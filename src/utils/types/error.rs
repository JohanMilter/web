use serde_json::error::Error as SJError;
use std::io::Error as IOError;
use tokio_tungstenite::tungstenite::Error as TTError;

#[derive(Debug)]
pub enum Error {
    InvalidRefference(String),
    CouldNotConnectToServer,
    BadStream,
    WebSocketError(TTError),
    IOError(IOError),
    SJError(SJError),
    ElementNotFound,
    InvalidCommand
}

impl From<TTError> for Error {
    fn from(error: TTError) -> Self {
        Error::WebSocketError(error)
    }
}
impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IOError(error)
    }
}
impl From<SJError> for Error {
    fn from(error: SJError) -> Self {
        Error::SJError(error)
    }
}
