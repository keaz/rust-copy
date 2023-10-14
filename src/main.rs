use std::{
    env,
    sync::{Arc, Mutex},
    thread, ops::ControlFlow,
};

use clap::Parser;
use console::{Emoji, style};
use rfcp::{
    cmd::CmdArgs,
    io::FileReader, SourceFile, get_reative_path, create_file_writer, create_progress_bars, copy_data, read_file_metadata, create_total_progressbar,
};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};


static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");


fn main() {
    let cmds = CmdArgs::parse_from(env::args_os());

    let file_reader = FileReader::new(cmds.source.clone());
    let size = file_reader.size();

    let source_files: Vec<SourceFile> = vec![];
    let file_data_arch = Arc::new(Mutex::new(source_files));

    let spinner_style = ProgressStyle::with_template("{.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let progress_bar = Arc::new(ProgressBar::new_spinner()) ;
    if let ControlFlow::Break(_) = read_file_metadata(file_reader, cmds.source.clone(), &file_data_arch, &progress_bar, spinner_style) {
        return;
    }
    progress_bar.finish_and_clear();

    let multi_progress = Arc::new(MultiProgress::new());


    let mut handlers = vec![];
    println!("{} {}Copying files...",style("[2/2]").bold().dim(),TRUCK);
    let total_file = Arc::new(Mutex::new(0));
    let total_size_tmp = Arc::new(Mutex::new(0));

    let total_size: u64 = file_data_arch.lock().unwrap().iter().map(|file_data| file_data.size).sum();
    let total_size_pb = create_total_progressbar(&multi_progress, total_size);
    let total_size_pb = Arc::new(total_size_pb);

    let buffer_size = cmds.buffer_size;

    for _ in 0..cmds.threads {
        let destination  = cmds.destination.clone();
        let file_data_arch =  file_data_arch.clone();
        let source = cmds.source.clone();
        let total_file = total_file.clone();
        let total_size_tmp = total_size_tmp.clone();
        let (total_size_pb, current_file) = create_progress_bars(&multi_progress, &total_size_pb);
        
        let handler = thread::spawn(move || {
        
            loop {
                let mut file_data = file_data_arch.lock().unwrap();
                if file_data.len() == 0 {
                    break;
                }
                let destination = destination.clone();
                if let Some(file) = file_data.pop() {
                    drop(file_data); // Drop the lock, so other threads can read the file_data

                    let relative_path = get_reative_path(&file, &source);
                    let mut reader = FileReader::from(file.file_path);
                    let name = reader.name();
                    current_file.set_message(format!("Copying file: {:?}",name));
                    let mut file_writer = create_file_writer(relative_path, name, destination, size);
                    
                    let mut offset = 0;
                    let mut buf = vec![0; buffer_size as usize];
                    loop  {
                        if let ControlFlow::Break(_) = copy_data(&mut reader, &mut offset, &mut buf, &mut file_writer, &total_size_pb, &total_size_tmp,buffer_size) {
                            break;
                        }
                    }
                    
                    let mut total_file = total_file.lock().unwrap();
                    *total_file = *total_file + 1;
                
                }
            }
        });
        handlers.push(handler);
        
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    total_size_pb.finish();
    let total_kb = (*total_size_tmp.lock().unwrap())/(1024 * 1024) ;
    println!("{} {} files copied, total KBs copied {} ",style(format!("{}",total_file.lock().unwrap())).bold().dim(),TRUCK,total_kb );

}




