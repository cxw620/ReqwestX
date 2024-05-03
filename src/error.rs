pub mod go_error;

#[derive(Debug, thiserror::Error)]
pub enum ErrorType {
    #[error("Unsupported HTTP method")]
    UnsupportedHttpMethod,
    #[error(transparent)]
    GoError(#[from] go_error::GoError),
}
