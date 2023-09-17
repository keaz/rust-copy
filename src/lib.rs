use std::{sync::{Arc, Mutex}, path::PathBuf, fs::{Metadata, self, ReadDir}, ops::ControlFlow};

use futures::{FutureExt, future::BoxFuture};
use indicatif::ProgressBar;

pub mod io;
pub mod cmd;

pub struct Data {
    pub data: Vec<u8>,
    pub offset: u64,
}


#[derive(Clone)]
pub struct SourceFile {
    pub file_path: PathBuf,
}


pub fn walk_dir(mut entries: fs::ReadDir, file_data: Arc<Mutex<Vec<SourceFile>>>, progress_bar: Arc<ProgressBar>){
    while let  Some(Ok(dir_entry)) = entries.next() {
        let path = dir_entry.path();
        
        if let Some(metadata) = folder_metadata(&path) {
            extract_detail_and_walk(metadata, path, file_data.clone(),progress_bar.clone());
        }
    }
    progress_bar.finish();
}

fn folder_metadata(path: &PathBuf) -> Option<Metadata> {
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
            data.push(SourceFile { file_path: path });
            return;
        }
    
        if metadata.is_dir() {
            progress_bar.set_prefix(format!("{:?}",path));
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