use std::{sync::{Arc, Mutex}, path::PathBuf, fs::{Metadata, self, ReadDir}, ops::ControlFlow, fmt::Write};


use console::{style, Emoji};
use indicatif::{ProgressBar, MultiProgress, ProgressStyle, ProgressState};
use io::{FileWriter, FileReader};

pub mod io;
pub mod cmd;

pub static DEFAULT_BUF_SIZE: u32 = 10240;
pub static DEFAULT_THREAD_COUNT:i8 = 3;
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");

pub struct Data {
    pub data: Vec<u8>,
    pub offset: u64,
}


#[derive(Clone)]
pub struct SourceFile {
    pub file_path: PathBuf,
    pub size: u64,
}



pub fn walk_dir(mut entries: fs::ReadDir, file_data: Arc<Mutex<Vec<SourceFile>>>, progress_bar: Arc<ProgressBar>){
    while let  Some(Ok(dir_entry)) = entries.next() {
        let path = dir_entry.path();
        
        if let Some(metadata) = folder_metadata(&path) {
            extract_detail_and_walk(metadata, path, file_data.clone(),progress_bar.clone());
        }
    }
}

pub fn folder_metadata(path: &PathBuf) -> Option<Metadata> {
    match fs::metadata(path) {
        Err(err) => {
            eprintln!("Error reading metadata {:?} error {:?}", path, err);
            return Option::None;
        }
        Ok(metadata) => Option::Some(metadata),
    }
}


fn extract_detail_and_walk(
    metadata: Metadata,
    path: PathBuf,
    file_data: Arc<Mutex<Vec<SourceFile>>>,progress_bar: Arc<ProgressBar>
)   {
    let progress_bar = progress_bar.clone();
   
        if metadata.is_file() {
            let mut data = (*file_data).lock().unwrap();
            data.push(SourceFile { file_path: path, size: metadata.len() });
            return;
        }
    
        if metadata.is_dir() {
            progress_bar.set_message(format!("Looking files in: {:?}",path));
            match fs::read_dir(&path) {
                Err(er) => {
                    eprintln!("Error reading directory {:?} error {:?}", path, er);
                }
                Ok(entries) => {
                    walk(entries, file_data,progress_bar.clone());
                }
            }
        }
   
    
}

fn walk(entries: ReadDir, file_data: Arc<Mutex<Vec<SourceFile>>>,progress_bar: Arc<ProgressBar>) {
    let file_data = file_data.clone();
    walk_dir(entries, file_data,progress_bar);
}

pub fn get_reative_path(file: &SourceFile, source: &String) -> String {
    let file_path  = file.file_path.clone();
    let source = PathBuf::from(source.clone());
    let relative_path = file_path.strip_prefix(source).unwrap();
    let relative_path = format!("{:?}",relative_path);
    let relative_path  = relative_path.replace("\"", "");
    relative_path
}

pub fn create_file_writer(relative_path: String, name: String, destination: String, size: u64) -> FileWriter {
    let parent_folder = relative_path.strip_suffix(&name).unwrap();
    let destination = PathBuf::from(destination).join(parent_folder);
    let file_writer = FileWriter::new(destination, name.clone(), size).unwrap();
    file_writer
}

pub fn copy_data(reader: &mut FileReader, offset: &mut u64, buf: &mut Vec<u8>, file_writer: &mut FileWriter, total_size_pb: &Arc<ProgressBar>, total_size_tmp: &Arc<Mutex<usize>>, buffer_size: u32) -> ControlFlow<()> {
    let dat_red = reader.read_random(*offset, buf).unwrap();
    if dat_red == 0 {
        return ControlFlow::Break(());
    }
    file_writer.write_random(*offset, &*buf).unwrap();
    total_size_pb.inc(dat_red as u64);
    let mut total_size = total_size_tmp.lock().unwrap();
    *total_size = *total_size + dat_red;
    *offset = *offset + buffer_size as u64;
    *buf = vec![0; buffer_size as usize];
                        
    ControlFlow::Continue(())
}

pub fn create_progress_bars(multi_progress: &Arc<MultiProgress>, total_size_pb: &Arc<ProgressBar>) -> (Arc<ProgressBar>, Arc<ProgressBar>) {
    let multi_progress = multi_progress.clone();
    let total_size_pb = total_size_pb.clone();
    let current_file = Arc::new(multi_progress.add(ProgressBar::new_spinner()));
    (total_size_pb, current_file)
}

pub fn read_file_metadata(file_reader: FileReader, source: String, file_data_arch: &Arc<Mutex<Vec<SourceFile>>>, progress_bar: &Arc<ProgressBar>, spinner_style: ProgressStyle) -> ControlFlow<()> {
    if file_reader.is_folder() {
        println!("{} {}Searching files...",style("[1/2]").bold().dim(),LOOKING_GLASS);
        let path = PathBuf::from(source);
        let reads = fs::read_dir(path);
        match reads {
            Err(er) => {
                eprintln!("Error reading folder  path {}",er);
                return ControlFlow::Break(());
            }
            Ok(entries) => {
                let file_data_arch = Arc::clone(file_data_arch);
                progress_bar.set_style(spinner_style.clone());
                walk_dir(entries, file_data_arch,progress_bar.clone());
            }
        }
    } else {
        let path_buff = PathBuf::new().join(source);
        let metadata = folder_metadata(&path_buff).unwrap();
        file_data_arch.lock().unwrap().push(SourceFile { file_path: path_buff,size: metadata.len() });
    }
    ControlFlow::Continue(())
}

pub fn create_total_progressbar(multi_progress: &Arc<MultiProgress>, total_size: u64) -> ProgressBar {
    let total_size_pb = multi_progress.add(ProgressBar::new(total_size));
    let sty = ProgressStyle::with_template(
        "[{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({elapsed_precise})",
    )
    .unwrap()
    .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
    .progress_chars("#>-");
    total_size_pb.set_style(sty);
    total_size_pb
}