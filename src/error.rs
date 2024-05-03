use thiserror::Error;

pub type PortableAudioLibraryResult<T> = std::result::Result<T, PortableAudioLibraryError>;

#[derive(Error, Debug)]
pub enum PortableAudioLibraryError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Bincode error: {0}")]
    SerdeError(#[from] bincode::Error),
}
