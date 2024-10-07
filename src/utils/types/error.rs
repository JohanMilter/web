use reqwest::Error as RQError;
use serde_json::error::Error as SJError;
use std::io::Error as IOError;
use tokio_tungstenite::tungstenite::Error as TTError;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    WebSocketError(TTError),
    IOError(IOError),
    SJError(SJError),
    RQError(RQError),
}

impl From<TTError> for Error {
    fn from(error: TTError) -> Self {
        Self::WebSocketError(error)
    }
}
impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Self::IOError(error)
    }
}
impl From<SJError> for Error {
    fn from(error: SJError) -> Self {
        Self::SJError(error)
    }
}
impl From<RQError> for Error {
    fn from(error: RQError) -> Self {
        Self::RQError(error)
    }
}

