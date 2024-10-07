use std::{future::Future, marker::PhantomData, sync::Arc};

use super::{
    behaviors::{TabRead, TabWrite},
    element::Element,
};
use crate::{
    logic::browser::drivers::behaviors::{DriverRead, DriverWrite},
    types::{by::By, wsstream::WSStream},
    utils::types::{error::Error, result::Result},
};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use tokio::{
    net::TcpStream,
    sync::{RwLock, RwLockWriteGuard},
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Debug, Clone)]
pub struct TabOptions {
    pub(crate) close_on_out_of_scope: bool,
    pub(crate) connect_on_init: bool,
    pub(crate) do_logging: bool,
}
impl Default for TabOptions {
    fn default() -> Self {
        Self {
            do_logging: true,
            connect_on_init: true,
            close_on_out_of_scope: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tab<D: DriverRead + DriverWrite> {
    #[serde(skip)]
    state: std::marker::PhantomData<D>,
    #[serde(skip)]
    pub(crate) stream: Option<Arc<RwLock<WSStream>>>,
    #[serde(skip)]
    pub(crate) options: TabOptions,
    pub(crate) description: String,
    #[serde(rename = "devtoolsFrontendUrl")]
    pub(crate) devtools_frontend_url: String,
    pub(crate) id: String,
    pub(crate) title: String,
    #[serde(rename = "type")]
    pub(crate) target_type: String,
    pub(crate) url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub(crate) web_socket_debugger_url: String,
}
impl<D: DriverRead + DriverWrite> Drop for Tab<D> {
    fn drop(&mut self) {
        if self.options.close_on_out_of_scope {
            //Close the tab
        }
    }
}
impl<D: DriverRead + DriverWrite> TabRead<D> for Tab<D> {
    async fn get_element(&self, by: By<'_>) -> Result<Element<D>> {
        let command = D::get_element(by);
        let response = self.send_command::<serde_json::Value>(command).await?;
        let object_id = response["result"]["result"]["objectId"]
            .as_str()
            .ok_or(Error::Custom("Could not get element".into()))?
            .to_string();

        let parent = match self.stream.as_ref() {
            Some(parent) => parent.clone(),
            None => return Err(Error::Custom("Could not get parent".into())),
        };

        Ok(Element::<D> {
            state: PhantomData::<D>,
            object_id,
            parent: Some(parent),
        })
    }
}
impl<D: DriverRead + DriverWrite> TabWrite<D> for Tab<D> {
    async fn send_command<T>(&self, command: serde_json::Value) -> Result<T>
    where
        T: DeserializeOwned, {
        let mut stream = match self.stream.as_ref() {
            Some(stream) => stream.write().await,
            None => return Err(Error::Custom("Could not send command".into())),
        };

        // Extract the command's ID
        let command_id = command
            .get("id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::Custom("Could not get the id from the command".into()))?;

        stream.0.send(Message::Text(command.to_string())).await?;

        while let Some(msg) = stream.1.next().await {
            let text = match msg {
                Ok(Message::Text(text)) => text,
                Ok(_) => continue,
                Err(e) => {
                    eprintln!("Error receiving message: {:?}", e);
                    continue;
                }
            };

            let value: serde_json::Value = match serde_json::from_str(&text) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Can't parse message as JSON: {:?}", e);
                    continue;
                }
            };

            match value.get("id").and_then(|v| v.as_i64()) {
                Some(id) => {
                    if id == command_id {
                        if self.options.do_logging {
                            println!("Received Response '{command_id}': \n{value}\n");
                        }
                        match serde_json::from_value::<T>(value) {
                            Ok(response) => return Ok(response),
                            Err(e) => eprintln!("Can't deserialize response: {:?}", e),
                        }
                    }
                }
                None => {
                    if self.options.do_logging {
                        println!("Received Async Event: \n{value}\n")
                    }
                }
            }
        }
        Err(Error::Custom("Could not send command".into()))
    }
    async fn re_attach_to_target(&self) -> Result<serde_json::Value> {
        self.send_command(D::tab_re_attach_to_target(&self.id))
            .await
    }
    async fn connect(&mut self) -> Result<()> {
        self.stream = Some(Arc::new(RwLock::new(
            connect_async(&self.web_socket_debugger_url)
                .await?
                .0
                .split(),
        )));
        Ok(())
    }
    async fn disconnect(&self) -> Result<()> {
        let mut stream = match self.stream.as_ref() {
            Some(stream) => stream.write().await,
            None => return Err(Error::Custom("Could not disconnect".into())),
        };

        stream.0.close().await?;

        Ok(())
    }

    async fn navigate(&self, url: &str) -> Result<serde_json::Value> {
        self.send_command(D::navigate(url)).await
    }
    async fn kill(&self) -> Result<serde_json::Value> {
        self.send_command(D::kill_tab(&self.id)).await
    }
    async fn refresh(&self) -> Result<serde_json::Value> {
        self.send_command(D::tab_refresh()).await
    }
    async fn back(&self, count: usize) -> Result<serde_json::Value> {
        assert!(
            count > 0,
            "Why the fuck would you use this function to go back 0 or less pages???"
        );
        _ = self.enable_page().await;

        let resp: serde_json::Value = self.send_command(D::get_navigation_history()).await?;

        let current_id = match resp["result"]["currentIndex"].as_u64() {
            Some(id) => id as usize,
            None => return Err(Error::Custom("Could not get current id".into())),
        };

        let entries = match resp["result"]["entries"].as_array() {
            Some(entries) => entries,
            None => return Err(Error::Custom("Could not get entries".into())),
        };

        let new_entry = match entries[current_id - count]["id"].as_u64() {
            Some(entry) => entry as u32,
            None => return Err(Error::Custom("Could not new entry".into())),
        };

        self.send_command(D::set_navigation_entry(new_entry)).await
    }
    async fn forward(&self, count: usize) -> Result<serde_json::Value> {
        assert!(
            count > 0,
            "Why the fuck would you use this function to go forward 0 or less pages???"
        );
        _ = self.enable_page().await;
        let resp: serde_json::Value = self.send_command(D::get_navigation_history()).await?;

        let current_id = match resp["result"]["currentIndex"].as_u64() {
            Some(id) => id as usize,
            None => return Err(Error::Custom("Could not get current id".into())),
        };

        let entries = match resp["result"]["entries"].as_array() {
            Some(entries) => entries,
            None => return Err(Error::Custom("Could not get entries".into())),
        };

        let new_entry = match entries[current_id + count]["id"].as_u64() {
            Some(entry) => entry as u32,
            None => return Err(Error::Custom("Could not new entry".into())),
        };

        self.send_command(D::set_navigation_entry(new_entry)).await
    }
    async fn enable_page(&self) -> Result<serde_json::Value> {
        self.send_command(D::enable_page()).await
    }
}
