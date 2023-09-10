mod local;

use crate::models::Folder;

pub trait StorageManager {
    fn save(&self, data: Folder);
    fn load(&self) -> Folder;
}