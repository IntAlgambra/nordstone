use std;

use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum ModelsError {
    #[error("Field already exists")]
    FieldAlreadyExist,

    #[error("Error opening file")]
    FileReadError(#[from] std::io::Error),

    #[error("Error retrieving filename")]
    GetFilenameError,

}
