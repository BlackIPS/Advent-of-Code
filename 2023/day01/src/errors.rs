use thiserror::Error;

#[derive(Error,Debug)]
pub enum AoCError {
    #[error("can not extract numbers from given line")]
    ExtractionError,
    
    #[error("An unknown or unhandled error occured")]
    UnknownError,
    #[error(transparent)]
    IoError(#[from] std::io::Error)
}