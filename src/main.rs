use std::{
    env,
    fmt::Write,
    path::{PathBuf},
    sync::{Arc, Mutex},
    thread, fs, time::Duration, ops::Range,
};

use clap::Parser;
use console::{Emoji, style};
use copy_file::{
    cmd::CmdArgs,
    io::{FileReader, FileWriter}, walk_dir, SourceFile, folder_metadata,
};
use indicatif::{ProgressBar, ProgressState, ProgressStyle, MultiProgress};

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static BUF_SIZE: usize = 10240;
static THREAD_COUNT: Range<i32> = 0..3;

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
    if file_reader.is_folder() {
        println!("{} {}Searching files...",style("[1/2]").bold().dim(),LOOKING_GLASS);
        let path = PathBuf::from(cmds.source.clone());
        let reads = fs::read_dir(path);
        match reads {
            Err(er) => {
                eprintln!("Error reading folder  path {}",er);
                return;
            }
            Ok(entries) => {
                let file_data_arch = Arc::clone(&file_data_arch);
                
                progress_bar.set_style(spinner_style.clone());
                walk_dir(entries, file_data_arch,progress_bar.clone());
            }
        }
    } else {
        let path_buff = PathBuf::new().join(&cmds.source.clone());
        let metadata = folder_metadata(&path_buff).unwrap();
        file_data_arch.lock().unwrap().push(SourceFile { file_path: path_buff,size: metadata.len() });
    }
    progress_bar.finish_and_clear();

    let multi_progress = Arc::new(MultiProgress::new());
    let sty = ProgressStyle::with_template(
        "[{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({elapsed_precise})",
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
    .progress_chars("#>-");

    let mut handlers = vec![];
    println!("{} {}Copying files...",style("[2/2]").bold().dim(),TRUCK);
    let total_file = Arc::new(Mutex::new(0));
    let total_size_tmp = Arc::new(Mutex::new(0));

    let total_size: u64 = file_data_arch.lock().unwrap().iter().map(|file_data| file_data.size).sum();
    let total_size_pb = multi_progress.add(ProgressBar::new(total_size));
    total_size_pb.set_style(sty);
    let total_size_pb = Arc::new(total_size_pb);

    for _ in THREAD_COUNT.clone() {
        let destination  = cmds.destination.clone();
        let file_data_arch =  file_data_arch.clone();
        let source = cmds.source.clone();
        let total_file = total_file.clone();
        let total_size_tmp = total_size_tmp.clone();
        let multi_progress = multi_progress.clone();
        let total_size_pb = total_size_pb.clone();
        let current_file = Arc::new(multi_progress.add(ProgressBar::new_spinner()));
        
        let handler = thread::spawn(move || {
        
            loop {
                let mut file_data = file_data_arch.lock().unwrap();
                if file_data.len() == 0 {
                    break;
                }
                let destination = destination.clone();
                if let Some(file) = file_data.pop() {
                    drop(file_data); // Drop the lock, so other threads can read the file_data

                    let file_path  = file.file_path.clone();
                    let source = PathBuf::from(source.clone());
                    let relative_path = file_path.strip_prefix(source).unwrap();
                    let relative_path = format!("{:?}",relative_path);
                    let relative_path  = relative_path.replace("\"", "");
                    
                    let mut reader = FileReader::from(file.file_path);
                    let name = reader.name();
                    current_file.set_message(format!("Copying file: {:?}",name));
                    let parent_folder = relative_path.strip_suffix(&name).unwrap();
                    let destination = PathBuf::from(destination).join(parent_folder);
                    let mut file_writer = FileWriter::new(destination, name.clone(), size).unwrap();
                    
                    let mut offset = 0;
                    let mut buf = vec![0; BUF_SIZE];
                    loop  {
                        let dat_red = reader.read_random(offset, &mut buf).unwrap();
                        if dat_red == 0 {
                            break;
                        }
                        file_writer.write_random(offset, &buf).unwrap();
                        total_size_pb.inc(dat_red as u64);
                        let mut total_size = total_size_tmp.lock().unwrap();
                        *total_size = *total_size + dat_red;
                        offset = offset + BUF_SIZE as u64;
                        buf = vec![0; BUF_SIZE];
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
