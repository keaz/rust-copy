use std::{
    env,
    fmt::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread, fs,
};

use clap::Parser;
use console::{Emoji, style};
use copy_file::{
    cmd::CmdArgs,
    io::{FileReader, FileWriter},
    Data, walk_dir, SourceFile,
};
use indicatif::{ProgressBar, ProgressState, ProgressStyle, MultiProgress};

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");

fn main() {
    let cmds = CmdArgs::parse_from(env::args_os());

    let file_reader = FileReader::new(cmds.source.clone());
    let size = file_reader.size();

    let source_files: Vec<SourceFile> = vec![];
    let file_data_arch = Arc::new(Mutex::new(source_files));


    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    
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
                let progress_bar = ProgressBar::new(10);
                progress_bar.set_style(spinner_style.clone());
                walk_dir(entries, file_data_arch,Arc::new(progress_bar));
            }
        }
    } else {
        file_data_arch.lock().unwrap().push(SourceFile { file_path: PathBuf::new().join(&cmds.source.clone()) });
    }

    let buf_size = 10240;
    
    let mut offset = 0;

    let m = Arc::new(MultiProgress::new());
    let sty = ProgressStyle::with_template(
        "{prefix:.green} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({elapsed_precise})",
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
    .progress_chars("#>-");
    let mut handlers = vec![];
    for i in 0..3 {
        let destination  = cmds.destination.clone();
        let file_data_arch =  file_data_arch.clone();
        let m  =  m.clone();
        let sty =  sty.clone();
        let source = cmds.source.clone();
        let handler = thread::spawn(move ||{
        
            loop {
                let mut file_data = file_data_arch.lock().unwrap();
                if file_data.len() == 0 {
                    break;
                }
                let destination = destination.clone();
                if let Some(file) = file_data.pop() {
                    drop(file_data);
                    let file_path  = file.file_path.clone();
                    let source = PathBuf::from(source.clone());
                    let relative_path = file_path.strip_prefix(source).unwrap();
                    let relative_path = format!("{:?}",relative_path);
                    let relative_path  = relative_path.replace("\"", "");
                    
                    let mut reader = FileReader::from(file.file_path);
                    let name = reader.name();
                    let parent_folder = relative_path.strip_suffix(&name).unwrap();
                    let destination = PathBuf::from(destination).join(parent_folder);
                    let mut file_writer = FileWriter::new(destination, name.clone(), size).unwrap();
    
                    let pb = m.add(ProgressBar::new(reader.size()));
                    pb.set_style(sty.clone());
                    pb.set_prefix(name);
    
                    let mut buf = vec![0; buf_size];
                    while reader.read_random(offset, &mut buf).unwrap() {
                        file_writer.write_random(offset, &buf).unwrap();
                        // pb.set_message(format!("{name}"));
                        pb.inc(buf.len() as u64);
    
                        offset = offset + buf_size as u64;
                        buf = vec![0; buf_size];
                        
                    }
                    pb.finish();
                }
            }
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

}
