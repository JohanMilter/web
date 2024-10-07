use core::panic;
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

use crate::{
    types::wsstream::WSStream,
    utils::types::{error::Error, result::Result},
};

pub mod behaviors;
pub mod drivers;
pub mod tools;

#[derive(Debug, Clone)]
pub struct BrowserOptions {
    pub(crate) close_on_out_of_scope: bool,
    pub(crate) connect_on_init: bool,
    pub(crate) do_logging: bool,
}
impl Default for BrowserOptions {
    fn default() -> Self {
        Self {
            close_on_out_of_scope: true,
            connect_on_init: true,
            do_logging: true,
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
            match &mut self.process {
                Some(process) => {
                    _ = process.start_kill();
                }
                None => panic!("[Drop Error] >> Browser already killed"),
            }
        }
    }
}
impl<D: DriverRead + DriverWrite> BrowserRead<D> for Browser<D> {
    async fn get_tabs(&self) -> Result<Vec<Tab<D>>>
    where
        Self: Sized, {
        match self.connection {
            Some(connection) => D::get_tabs(&connection).await,
            None => Err(Error::Custom("Could not get tabs".into())),
        }
    }
}

impl<D: DriverRead + DriverWrite> BrowserWrite<D> for Browser<D> {
    async fn open(port: u16, options: Option<BrowserOptions>) -> Result<(Self, Tab<D>)>
    where
        Self: Sized, {
        const LOCAL_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

        let process = Some(D::open_process(LOCAL_ADDRESS, port));
        let url = format!("http://{LOCAL_ADDRESS}:{port}/json/version");
        let resp = reqwest::get(&url).await?;
        let mut this = resp.json::<Browser<D>>().await?;
        this.connection = Some((LOCAL_ADDRESS, port));
        this.process = process;
        this.state = std::marker::PhantomData::<D>;
        this.options = options.unwrap_or_default();
        this.connect().await?;

        let mut tabs = this.get_tabs().await?;
        let mut first_tab = tabs.remove(0);
        drop(tabs);

        if this.options.connect_on_init {
            first_tab.connect().await?;
        }

        // Set the settings
        first_tab.options.do_logging = this.options.do_logging;
        first_tab.options.close_on_out_of_scope = this.options.close_on_out_of_scope;
        first_tab.options.connect_on_init = this.options.connect_on_init;

        Ok((this, first_tab))
    }
    async fn send_command<T>(&mut self, command: serde_json::Value) -> Result<T>
    where
        T: DeserializeOwned, {
        let stream = match self.stream.as_mut() {
            Some(stream) => stream,
            None => return Err(Error::Custom("Could send command".into())),
        };

        stream.0.send(Message::Text(command.to_string())).await?;

        let match_command_id = command
            .get("id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::Custom("Could not get id from command".into()))?;

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
                    if id == match_command_id {
                        if self.options.do_logging {
                            println!("Received Response '{match_command_id}': \n{value}");
                        }
                        match serde_json::from_value::<T>(value) {
                            Ok(response) => return Ok(response),
                            Err(e) => eprintln!("Can't deserialize response: {:?}", e),
                        }
                    }
                }
                None => {
                    if self.options.do_logging {
                        println!("Received Async Event: \n{value}")
                    }
                }
            }
        }
        Err(Error::Custom("Could not send the command".into()))
    }
    async fn new_tab(&mut self, options: Option<TabOptions>) -> Result<Tab<D>> {
        let resp = self.send_command::<serde_json::Value>(D::new_tab()).await?;

        let lookup_id = resp["result"]["targetId"]
            .as_str()
            .ok_or(Error::Custom("Could not get lookup id".into()))?
            .to_string();

        let mut tab = match self
            .get_tabs()
            .await?
            .into_iter()
            .find(|tab| tab.id == lookup_id) {
                Some(tab) => tab,
                None => return Err(Error::Custom("Could not get tab".into()))
            };

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
        let stream = match self.stream.as_mut(){
            Some(stream) => stream,
            None => return Err(Error::Custom("Could not disconnect".into()))
        };
        stream.0.close().await?;
        Ok(())
    }
}
