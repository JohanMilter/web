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
use tokio::{
    net::TcpStream,
    sync::{RwLock, RwLockWriteGuard},
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Debug, Clone)]
pub struct TabOptions {
    pub(crate) close_on_out_of_scope: bool,
    pub(crate) connect_on_init: bool,
}
impl Default for TabOptions {
    fn default() -> Self {
        Self {
            close_on_out_of_scope: true,
            connect_on_init: true,
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
            .ok_or(Error::ElementNotFound)?
            .to_string();
        Ok(Element::<D> {
            state: PhantomData::<D>,
            object_id,
            parent: Some(self.stream.as_ref().unwrap().clone()),
        })
    }
}
impl<D: DriverRead + DriverWrite> TabWrite<D> for Tab<D> {
    async fn send_command<T>(&self, command: serde_json::Value) -> Result<T>
    where
        T: DeserializeOwned, {
        let mut stream = self.stream.as_ref().unwrap().write().await;
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
        Err(Error::BadStream)
    }
    async fn send_command_with_session<T>(
        &self,
        session_id: &str,
        mut command: serde_json::Value,
    ) -> Result<T>
    where
        T: DeserializeOwned, {
        command["sessionId"] = serde_json::Value::String(session_id.to_string());

        let mut stream = self.stream.as_ref().unwrap().write().await;
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
        Err(Error::BadStream)
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
        let mut stream = self.stream.as_ref().unwrap().write().await;
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
    async fn back(&self) -> Result<serde_json::Value> {
        let resp = self.re_attach_to_target().await.unwrap();
        let session_id = &resp["params"]["sessionId"];
        let history = self.send_command(D::get_navigation_history()).await;
        println!("back:resp = {:?}", history);
        history
    }
    async fn forward(&self) -> Result<serde_json::Value> {
        let history: serde_json::Value = self
            .send_command(D::get_navigation_history())
            .await
            .unwrap();
        println!("forward:history = {history}");
        todo!()
    }
}
