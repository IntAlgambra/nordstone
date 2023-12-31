use std::io::{Read, Write};
use age;
use age::secrecy::Secret;
use bincode;

use crate::encryption::Encryptor;
use crate::models::Folder;

pub struct AgeEncryptor {
    key: String,
}

impl AgeEncryptor {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Encryptor for AgeEncryptor {
    fn encrypt(&self, data: &mut Folder) -> Vec<u8> {
        let bytes_data = bincode::serialize(&data).unwrap();
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(self.key.clone()));
        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
        writer.write_all(&bytes_data).unwrap();
        writer.finish().unwrap();
        encrypted
    }

    fn decrypt(&self, data: Vec<u8>) -> Folder {
        let decryptor = match age::Decryptor::new(&data[..]).unwrap() {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!()
        };
        let mut decrypted = Vec::new();
        let mut reader = decryptor.decrypt(&Secret::new(self.key.clone()), None).unwrap();
        reader.read_to_end(&mut decrypted).unwrap();
        let mut folder: Folder = bincode::deserialize(&decrypted).unwrap();
        folder
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::encryption::age_encryptor::AgeEncryptor;
    use crate::encryption::Encryptor;
    use crate::models::{Folder, Record};

    #[test]
    fn test_encryptor() {
        let encryptor = AgeEncryptor::new("key".into());
        let mut record = Record::new();
        record.add_field("name".into(), "value".into()).unwrap();
        let path = Path::new("tests/testfile.test");
        record.add_file(&path).unwrap();
        let mut subfolder = Folder::new();
        let mut main_folder = Folder::new();
        main_folder.add_folder(subfolder);
        main_folder.add_record(record);
        let key = String::from("key");
        let encrypted = encryptor.encrypt(main_folder);
        let decrypted = encryptor.decrypt(encrypted);
    }
}