#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Manifest parse error: {0}")]
    ManifestParse(String),

    #[error("Manifest validation failed: {0}")]
    ManifestValidation(String),

    #[error("Invalid resource: {0}")]
    InvalidResource(String),

    #[error("Filesystem error: {0}")]
    Filesystem(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Hash mismatch: {0}")]
    HashMismatch(String),

    #[error("File conflict: {0}")]
    Conflict(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}
