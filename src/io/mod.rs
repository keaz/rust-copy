// use std::{path::{Path, PathBuf}, fs::{File, self, remove_file}, io::{SeekFrom, Seek, Write, Read}};

use std::{path::{PathBuf, Path}, io::{SeekFrom, Error, Seek, Read, Write}, fs::{File, remove_file, self}};

use log::{warn, debug};

const BUFFER_SIZE: u64 = 1024 * 1024 * 50;

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
            eprintln!("Error opeining file {}",er);
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
    )  -> Result<bool,FileError> {
        
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        let read_data = self.file.read(buf).unwrap();

        Ok(read_data > 0)
    }
}

pub struct FileWriter {
    file: File,
}

impl FileWriter {

    pub fn new(destination: PathBuf, file_name: String, size: u64) -> Result<Self,FileError> {
        if !destination.exists() {
            fs::create_dir_all(&destination).unwrap();
        }
        let path = Path::new(&destination).join(&file_name);
        let path_exists = path.exists();

        if path_exists {
            warn!("File exists deleting the file {}",path_exists);
            let _ = remove_file(&path);
        }

        let file = match File::create(path.clone()) {
            Ok(new_file) => new_file,
            Err(err) => {
                let error_message = err.to_string();
                return Err(FileError::CannotCreate(error_message));
            },
        };

        debug!("New file created {:?}",file);
        // let file = Self::write_random_data(file,size)?;
        Ok(FileWriter {file})
    }
    

     fn write_random_data(file: File, size: u64) -> Result<File, FileError> {
        let mut file = file;
        if size > BUFFER_SIZE  {
            let mut offset = 0;
            let buffers = size / BUFFER_SIZE ;
            let last_buffer = size % BUFFER_SIZE;
            let buffer = vec![0;BUFFER_SIZE as usize];
            let mut buffer_index = 0;
            
            while  buffer_index < buffers {
                file = Self::write_random_to_given_file(file, offset +1 , &buffer)?;
                offset += BUFFER_SIZE as u64 ;
                buffer_index = buffer_index + 1;
            }

            if last_buffer != 0 {
                let buffer = vec![0;last_buffer as usize];
                file =  Self::write_random_to_given_file(file,offset +1 , &buffer)?;
            }
        }

        Ok(file)
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
        self.file.seek(SeekFrom::Start(offset)).unwrap();
        self.file.write(buf).unwrap();
        Ok(())
    }

    pub  fn write_random_to_given_file(mut file: File, offset: u64, buf: &[u8]) -> Result<File,FileError>{
        file.seek(SeekFrom::Start(offset)).unwrap();
        file.write(buf).unwrap();
        Ok(file)
    }

}