mod errors;

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::{Deserialize, Serialize};

use errors::ModelsError;

#[derive(Serialize, Deserialize)]
pub struct RecordFile {
    filename: OsString,
    extension: OsString,
    content: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    fields: HashMap<String, String>,
    files: Option<Vec<RecordFile>>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            files: None,
        }
    }

    pub fn add_field(&mut self, field_name: String, value: String) -> Result<(), ModelsError> {
        if self.fields.contains_key(&field_name) {
            return Err(ModelsError::FieldAlreadyExist);
        }
        self.fields.insert(field_name, value);
        Ok(())
    }

    pub fn add_file(&mut self, file_path: &Path) -> Result<(), ModelsError> {
        let mut file = File::open(file_path)?;
        let filename = match file_path.file_name() {
            Some(name) => name.to_os_string(),
            None => {
                return Err(ModelsError::GetFilenameError);
            }
        };
        let extension = match file_path.extension() {
            Some(ext) => ext.to_os_string(),
            None => {
                return Err(ModelsError::GetFilenameError);
            }
        };
        let mut buf = Vec::new();
        let _ = file.read(buf.as_mut_slice())?;
        let record_file = RecordFile {
            filename,
            extension,
            content: buf,
        };
        match self.files.as_mut() {
            Some(mut files) => files.push(record_file),
            None => {
                let mut files = Vec::new();
                files.push(record_file);
                self.files = Some(files);
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Folder {
    records: Vec<Record>,
    subfolders: Option<Vec<Box<Folder>>>,
}

impl Folder {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            subfolders: None,
        }
    }

    pub fn add_record(&mut self, record: Record) {
        self.records.push(record);
    }

    pub fn add_folder(&mut self, folder: Self) {
        let boxed = Box::new(folder);
        match self.subfolders.as_mut() {
            Some(subfolders) => subfolders.push(boxed),
            None => {
                let mut subfolders = Vec::new();
                subfolders.push(boxed);
                self.subfolders = Some(subfolders);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Folder, Record};
    use std::path::Path;

    fn create_record() -> Record {
        let mut record = Record::new();
        record
            .add_field("domain".into(), "yandex.ru".into())
            .unwrap();
        let path = Path::new("../../tests/testfile.test");
        record.add_file(&path).unwrap();
        record
    }

    #[test]
    fn test_create_record() {
        create_record();
    }

    #[test]
    fn test_create_folder() {
        let mut folder = Folder::new();
        let record = create_record();
        folder.add_record(record);
        let mut subfolder = Folder::new();
        let record = create_record();
        subfolder.add_record(record);
        folder.add_folder(subfolder);
    }


}
