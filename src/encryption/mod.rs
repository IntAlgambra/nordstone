mod age_encryptor;

use crate::models::Folder;

pub use age_encryptor::AgeEncryptor;

pub trait Encryptor {
    fn encrypt(&self, data: &mut Folder) -> Vec<u8>;
    fn decrypt(&self, data: Vec<u8>) -> Folder;
}