mod local;

use crate::models::Folder;

pub use local::LocalStorageManager;

pub trait StorageManager {
    fn save(&self, data: &mut Folder);
    fn load(&self) -> Folder;
}