use std::path::PathBuf;
use std::fs;

use crate::models::Folder;
use crate::storage::StorageManager;
use crate::encryption::{AgeEncryptor, Encryptor};

const EXTENSION: &'static str = "nrdstn";

struct LocalStorageManager {
    path: PathBuf,
    encryptor: AgeEncryptor,
}

impl LocalStorageManager {
    fn new(path: PathBuf, encryptor: AgeEncryptor) -> Self {
        Self {
            path,
            encryptor,
        }
    }
}

impl StorageManager for LocalStorageManager {
    fn save(&self, data: Folder) {
        let encrypted_data = self.encryptor.encrypt(data);
        fs::write(&self.path, encrypted_data).unwrap();
    }

    fn load(&self) -> Folder {
        let encrypted_data = fs::read(&self.path).unwrap();
        self.encryptor.decrypt(encrypted_data)
    }
}