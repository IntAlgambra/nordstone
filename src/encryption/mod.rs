mod age_encryptor;

use crate::models::Folder;


pub trait Encryptor {
    fn encrypt(&self, data: Folder) -> Vec<u8>;
    fn decrypt(&self, data: Vec<u8>) -> Folder;
}