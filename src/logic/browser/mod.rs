use std::net::Ipv4Addr;

use behaviors::{BrowserRead, BrowserWrite};
use drivers::behaviors::{DriverRead, DriverWrite};
use futures::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tools::{
    behaviors::{TabRead, TabWrite},
    tab::{Tab, TabOptions},
};

use crate::utils::types::{error::Error, result::Result};

use super::WSStream;

pub mod behaviors;
pub mod drivers;
pub mod tools;

#[derive(Debug, Clone)]
pub struct BrowserOptions {
    pub(crate) close_on_out_of_scope: bool,
    pub(crate) connect_on_init: bool,
}
impl Default for BrowserOptions {
    fn default() -> Self {
        Self {
            close_on_out_of_scope: true,
            connect_on_init: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Browser<D: DriverRead + DriverWrite> {
    #[serde(rename = "webSocketDebuggerUrl")]
    pub(crate) web_socket_debugger_url: String,
    #[serde(skip)]
    state: std::marker::PhantomData<D>,
    #[serde(skip)]
    pub(crate) stream: Option<WSStream>,
    #[serde(skip)]
    pub(crate) options: BrowserOptions,
    #[serde(skip)]
    pub(crate) process: Option<tokio::process::Child>,
    #[serde(skip)]
    pub(crate) connection: Option<(Ipv4Addr, u16)>,
}
impl<D: DriverRead + DriverWrite> Drop for Browser<D> {
    fn drop(&mut self) {
        if self.options.close_on_out_of_scope {
            _ = self.process.as_mut().unwrap().start_kill();
        }
    }
}
impl<D: DriverRead + DriverWrite> BrowserRead<D> for Browser<D> {
    async fn get_tabs(&self) -> Result<Vec<Tab<D>>>
    where
        Self: Sized, {
        D::get_tabs(self.connection.unwrap()).await
    }
}

impl<D: DriverRead + DriverWrite> BrowserWrite<D> for Browser<D> {
    async fn open(port: u16, options: Option<BrowserOptions>) -> Result<(Self, Tab<D>)>
    where
        Self: Sized, {
        const LOCAL_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

        let process = Some(D::open_process(LOCAL_ADDRESS, port));
        let url = format!("http://{LOCAL_ADDRESS}:{port}/json/version");
        let resp = reqwest::get(&url).await.unwrap();
        let mut this = resp.json::<Browser<D>>().await.unwrap();
        this.connection = Some((LOCAL_ADDRESS, port));
        this.process = process;
        this.state = std::marker::PhantomData::<D>;
        this.options = options.unwrap_or_default();
        this.connect().await?;

        let mut first_tab = this.get_tabs().await.unwrap().remove(0);
        if this.options.connect_on_init {
            first_tab.connect().await?;
        }
        Ok((this, first_tab))
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
                Ok(Message::Text(text)) => match serde_json::from_str::<T>(&text) {
                    Ok(response) => {
                        return Ok(response);
                    }
                    Err(_) => eprintln!("Can't deserialize:\n{text}\n"),
                },
                Ok(_) => continue,
                Err(_) => return Err(Error::BadStream),
            }
        }
        Err(Error::BadStream)
    }
    async fn new_tab(&mut self, options: Option<TabOptions>) -> Result<Tab<D>> {
        let resp = self
            .send_command::<serde_json::Value>(D::new_tab())
            .await
            .unwrap();
        let lookup_id = resp["result"]["targetId"]
            .as_str()
            .ok_or(Error::ElementNotFound)?
            .to_string();
        let mut tab = self
            .get_tabs()
            .await
            .unwrap()
            .into_iter()
            .find(|tab| tab.id == lookup_id)
            .unwrap();
        tab.options = options.unwrap_or_default();
        if tab.options.connect_on_init {
            _ = tab.connect().await;
        }
        Ok(tab)
    }
    async fn connect(&mut self) -> Result<()> {
        let (ws_stream, _) = connect_async(&self.web_socket_debugger_url).await?;
        self.stream = Some(ws_stream.split());
        Ok(())
    }
    async fn disconnect(&mut self) -> Result<()> {
        let stream = self.stream.as_mut().unwrap();
        stream.0.close().await?;
        Ok(())
    }
}
