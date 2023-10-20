use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
    time::SystemTime,
};

use filetime::{set_file_mtime, FileTime};
use log::warn;

#[derive(Debug)]
pub enum FileError {
    CannotCreate(String),
    FileNotCreate(String),
}

pub struct FileReader {
    file: File,
    file_name: String,
}

impl FileReader {
    pub fn new(path: String) -> Self {
        let path_buf = PathBuf::new().join(&path);
        if !path_buf.exists() {
            warn!("File does not exists {:?}", path);
        }
        let cl = path_buf.clone();
        let file_name = cl.file_name().unwrap().to_str().unwrap();
        FileReader {
            file: File::open(path_buf).unwrap(),
            file_name: String::from(file_name),
        }
    }

    pub fn from(path_buf: PathBuf) -> Self {
        if !path_buf.exists() {
            warn!("File does not exists {:?}", path_buf);
        }
        let cl = path_buf.clone();
        let file_name = cl.file_name().unwrap().to_str().unwrap();
        let file = File::open(path_buf);
        if let Err(er) = file {
            eprintln!("Error opening file {}", er);
            panic!()
        }
        FileReader {
            file: file.unwrap(),
            file_name: String::from(file_name),
        }
    }
}

impl FileReader {
    pub fn name(&self) -> String {
        self.file_name.clone()
    }

    pub fn size(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    pub fn is_folder(&self) -> bool {
        self.file.metadata().unwrap().is_dir()
    }

    pub fn read_random(&mut self, offset: u64, buf: &mut [u8]) -> Result<usize, FileError> {
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let read_data = self.file.read(buf).unwrap();

        Ok(read_data)
    }
}

pub struct FileWriter {
    file: File,
    path: PathBuf,
    source_modified: Option<SystemTime>,
}

impl FileWriter {
    pub fn new(
        file: File,
        path: PathBuf,
        source_modified: Option<SystemTime>,
    ) -> Result<Self, FileError> {
        return Ok(FileWriter {
            file: file,
            path,
            source_modified,
        });
    }

    pub fn set_modified(&self) {
        if let Some(source_time) = self.source_modified {
            set_file_mtime(self.path.clone(), FileTime::from_system_time(source_time)).unwrap();
        }
    }

    ///
    ///    Delete a file in the root folder
    ///    # Arguments
    ///    * `root_folder` - The root folder where the file will be deleted
    ///    * `file_name` - The relative path to the root folder and name of the file to be deleted
    ///
    // pub fn delete_file(self) {
    //     fs::remove_file(self.path).unwrap();
    // }

    ///
    ///    Delete a folder in the root folder
    ///    # Arguments
    ///    * `root_folder` - The root folder where the folder will be deleted
    ///    * `folder_name` - The relative path to the root folder and name of the folder to be deleted
    ///    
    // pub fn delete_folder(self){
    //     fs::remove_dir_all(self.path).unwrap();
    // }

    ///
    ///        Writes a random chunk of data to a file
    ///   # Arguments
    ///        * `root_folder` - The root folder where the file will be written
    ///        * `file_name` - The relative path to the root folder and name of the file to be written
    ///        * `offset` - The offset from where to start writing
    ///        * `buf` - The buffer where the data will be written
    ///
    pub fn write_random(&mut self, offset: u64, buf: &[u8]) -> Result<(), FileError> {
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        self.file.write(buf).unwrap();
        Ok(())
    }
}
