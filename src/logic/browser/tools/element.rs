use std::sync::Arc;

use futures::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    logic::browser::drivers::behaviors::{DriverRead, DriverWrite},
    types::{error::Error, result::Result, wsstream::WSStream},
};

use super::{
    behaviors::{ElementRead, ElementWrite, TabWrite},
    tab::Tab,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element<D: DriverRead + DriverWrite> {
    #[serde(skip)]
    pub(crate) parent: Option<Arc<RwLock<WSStream>>>,
    #[serde(skip)]
    pub(crate) state: std::marker::PhantomData<D>,
    pub(crate) object_id: String,
}
impl<D: DriverRead + DriverWrite> ElementRead<D> for Element<D> {
    async fn get_text(&self) -> Result<serde_json::Value> {
        self.send_command(D::get_text(&self.object_id)).await
    }
    async fn send_command<T>(&self, command: serde_json::Value) -> Result<T>
    where
        T: DeserializeOwned, {
        let mut stream = match self.parent.as_ref() {
            Some(stream) => stream.write().await,
            None => return Err(Error::Custom("Could not send command".into())),
        };

        stream.0.send(Message::Text(command.to_string())).await?;

        while let Some(msg) = stream.1.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("Received msg: \n{text}");
                    match serde_json::from_str::<T>(&text) {
                        Ok(response) => return Ok(response),
                        Err(_) => eprintln!("Can't deserialize:\n{text}\n"),
                    }
                }
                Ok(_) => continue,
                Err(_) => eprintln!("Can't deserialize:\n{:?}\n", msg),
            }
        }
        Err(Error::Custom("Could not send command".into()))
    }
}
impl<D: DriverRead + DriverWrite> ElementWrite<D> for Element<D> {
    async fn click(&self) -> Result<serde_json::Value> {
        self.send_command(D::click_element(&self.object_id)).await
    }
    async fn focus(&self) -> Result<serde_json::Value> {
        self.send_command(D::focus(&self.object_id)).await
    }
    async fn set_text(&self, text: &str) -> Result<serde_json::Value> {
        _ = self.focus().await;
        self.send_command(D::set_text(text)).await
    }
}
