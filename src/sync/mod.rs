mod telegram;

use crate::models::Folder;

pub trait SyncManager {
    fn upload(&self, folder: Folder);
    fn download(&self) -> Folder;
    fn merge(&self, remote_data: Folder, local_folder: Folder) -> Folder;
}