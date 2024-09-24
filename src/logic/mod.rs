use futures::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

#[cfg(feature = "browser")]
pub mod browser;

#[cfg(feature = "crawler")]
pub mod crawler;

#[cfg(feature = "scraper")]
pub mod scraper;
