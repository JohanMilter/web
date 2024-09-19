use std::{future::Future, marker::PhantomData};

use super::{
    behaviors::{TabRead, TabWrite},
    element::Element,
};
use crate::{
    logic::{
        browser::drivers::behaviors::{DriverRead, DriverWrite},
        WSStream,
    },
    types::by::By,
    utils::types::{error::Error, result::Result},
};
use futures::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};

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
    pub(crate) stream: Option<WSStream>,
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
    async fn get_element(&mut self, by: By<'_>) -> Result<Element<D>> {
        let command = D::runtime_evaluate(D::get_element(by));
        let response = self.send_command::<serde_json::Value>(command).await?;
        let object_id = response["result"]["result"]["objectId"]
            .as_str()
            .ok_or(Error::ElementNotFound)?
            .to_string();
        Ok(Element::<D> {
            state: PhantomData::<D>,
            object_id,
        })
    }
}
impl<D: DriverRead + DriverWrite> TabWrite<D> for Tab<D> {
    async fn connect(&mut self) -> Result<()> {
        self.stream = Some(
            connect_async(&self.web_socket_debugger_url)
                .await?
                .0
                .split(),
        );
        Ok(())
    }
    async fn disconnect(&mut self) -> Result<()> {
        let stream = self.stream.as_mut().unwrap();
        stream.0.close().await?;
        Ok(())
    }
    async fn send_command<T>(&mut self, command: serde_json::Value) -> Result<T>
    where
        T: DeserializeOwned, {
        self.stream
            .as_mut()
            .unwrap()
            .0
            .send(Message::Text(command.to_string()))
            .await?;
        while let Some(msg) = self.stream.as_mut().unwrap().1.next().await {
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
    async fn navigate(&mut self, url: &str) -> Result<serde_json::Value> {
        self.send_command(D::navigate(url)).await
    }
    async fn kill(&mut self) -> Result<serde_json::Value> {
        self.send_command(D::kill_tab(&self.id)).await
    }
}
