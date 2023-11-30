use thiserror::Error;

#[derive(Error,Debug)]
pub enum AoCError {
    #[error("An unknown or unhandled error occured")]
    UnknownError,
    #[error(transparent)]
    IoError(#[from] std::io::Error)
}