mod age_encryptor;

use crate::models::Folder;


pub trait Encryptor {
    fn encrypt(&self, key: String, data: Folder) -> Vec<u8>;
    fn decrypt(&self, key: String, data: Vec<u8>) -> Folder;
}