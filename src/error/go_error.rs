#[derive(Debug, thiserror::Error)]
pub enum GoError {
    #[error("GoError: Client not initialized")]
    ClientNotInitialized,
    #[error("GoError: Invalid request method")]
    InvalidRequestMethod,
    #[error("GoError: update_impersonation_config: unknown extension type {0}")]
    InvalidTlsExtension(String),
    #[error("GoError: Unknown: {0}")]
    Unknown(String),
}

impl From<(i32, String)> for GoError {
    fn from((code, message): (i32, String)) -> Self {
        match code {
            -1_000_000 => GoError::ClientNotInitialized,
            -1_000_010 => GoError::InvalidRequestMethod,
            -1_001_001 => GoError::InvalidTlsExtension(message),
            _ => GoError::Unknown(message),
        }
    }
}
