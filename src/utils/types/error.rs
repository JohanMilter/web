use tokio_tungstenite::tungstenite::Error as TTError;
use std::io::Error as IOError;

#[derive(Debug)]
pub enum Error
{
    InvalidRefference(String),
    CouldNotConnectToServer,
    BadStream,
    WebSocketError(TTError),
    IOError(IOError)
}

impl From<TTError> for Error {
    fn from(error: TTError) -> Self {
        Error::WebSocketError(error)
    }
}
impl From<IOError> for Error{
    fn from(error: IOError) -> Self {
        Error::IOError(error)
    }
}