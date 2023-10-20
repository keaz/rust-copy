use std::{path::{PathBuf, Path}, io::{SeekFrom, Seek, Read, Write}, fs::{File, remove_file, self, Metadata}, time::SystemTime};

use filetime::{set_file_mtime, FileTime};
use log::{warn, debug};

use crate::folder_metadata;

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
            warn!("File does not exists {:?}",path);
        }
        let cl = path_buf.clone();
        let file_name = cl
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        FileReader { file: File::open(path_buf).unwrap(), file_name: String::from(file_name) }
    }

    pub fn from(path_buf: PathBuf) -> Self{
        if !path_buf.exists() {
            warn!("File does not exists {:?}",path_buf);
        }
        let cl = path_buf.clone();
        let file_name = cl
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let file = File::open(path_buf);
        if let Err(er) =  file {
            eprintln!("Error opening file {}",er);
            panic!()
        }
        FileReader { file: file.unwrap(), file_name: String::from(file_name) }
    }
}


impl FileReader {

  

    ///
    /// Create a file in the root folder
    /// # Arguments
    /// * `root_folder` - The root folder where the file will be created
    /// * `file_name` - The relative path to the root folder and name of the file to be created
    ///
    // pub fn create_file(&mut self, size: u64) -> Result<(),FileError> {

    //     let path_exists = self.path.exists();

    //     if path_exists {
    //         warn!("File exists deleting the file {}",path_exists);
    //         let _ = remove_file(&self.path);
    //     }
    //     let new_file = match File::create(self.path.clone()) {
    //         Ok(new_file) => new_file,
    //         Err(err) => {
    //             let error_message = err.to_string();
    //             return Err(FileError::CannotCreate(error_message));
    //         },
    //     };

    //     debug!("New file created {:?}",new_file);
    //     self.file = Option::Some(new_file);
    //     self.write_random_data(size)?;
    //     Ok(())
    // }


    pub fn name(&self) -> String {
        self.file_name.clone()
    }
    
    pub fn size(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    pub fn is_folder(&self) -> bool {
        self.file.metadata().unwrap().is_dir()
    }

    pub fn read_random(
        &mut self,
        offset: u64,
        buf: &mut [u8],
    )  -> Result<usize,FileError> {
        
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let read_data = self.file.read(buf).unwrap();

        Ok(read_data)
    }
}

pub struct FileWriter {
    file: Option<File>,
    is_copied: bool,
    path: PathBuf,
    source_modified: Option<SystemTime>
}

impl FileWriter {

    pub fn new(destination: PathBuf, file_name: String, _size: u64, source_modified: Option<SystemTime>) -> Result<Self,FileError> {
        if !destination.exists() {
            fs::create_dir_all(&destination).unwrap();
        }
        let path = Path::new(&destination).join(&file_name);
        let path_exists = path.exists();

        let mut is_copied = false;
        if path_exists {
            if let Some(source_modified) = source_modified {
                if let Some(metadata) = folder_metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        is_copied = modified.eq(&source_modified);
                    }
                }
            }
            
            if is_copied {
                return Ok(FileWriter {file: None,is_copied, path, source_modified})
            }
            
            warn!("File exists and modified, deleting the file {}",path_exists);
            let _ = remove_file(&path);
            let file = match File::create(path.clone()) {
                Ok(new_file) => new_file,
                Err(err) => {
                    let error_message = err.to_string();
                    return Err(FileError::CannotCreate(error_message));
                },
            };
            debug!("New file created {:?}",file);
            return Ok(FileWriter {file: Some(file),is_copied, path, source_modified});
        }

        let file = match File::create(path.clone()) {
            Ok(new_file) => new_file,
            Err(err) => {
                let error_message = err.to_string();
                return Err(FileError::CannotCreate(error_message));
            },
        };
        debug!("New file created {:?}",file);
        return Ok(FileWriter {file: Some(file),is_copied, path, source_modified});
    }

    pub fn is_copied(&self)-> bool {
        self.is_copied
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
    pub  fn write_random(&mut self, offset: u64, buf: &[u8]) -> Result<(),FileError>{
        let file = self.file.as_mut().unwrap();
        file.seek(SeekFrom::Start(offset)).unwrap();
        file.write(buf).unwrap();
        Ok(())
    }

    pub  fn write_random_to_given_file(mut file: File, offset: u64, buf: &[u8]) -> Result<File,FileError>{
        file.seek(SeekFrom::Start(offset)).unwrap();
        file.write(buf).unwrap();
        Ok(file)
    }

}
