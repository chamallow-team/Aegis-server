#[derive(thiserror::Error, Debug)]
pub enum PackError {
    #[error("Archive is empty")]
    EmptyArchive,
    #[error("File 'config.mp' is missing")]
    HeaderIsMissing,
    #[error("Cannot read 'config.mp' file")]
    CannotReadHeader(#[from] std::io::Error),
    #[error("Invalid 'config.mp' file")]
    InvalidHeader(#[from] rmp_serde::decode::Error),

    #[error("Invalid mod ID")]
    InvalidModID,
}
