use thiserror::Error;
#[derive(Debug, Error)] pub enum CoreError { #[error("unsupported format: {0}")] UnsupportedFormat(String), #[error("invalid path: {0}")] InvalidPath(String), #[error("conversion failed: {0}")] ConversionFailed(String), #[error("io error: {0}")] Io(String), #[error("cancelled")] Cancelled }
