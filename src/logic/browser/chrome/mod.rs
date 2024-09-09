use std::error::Error;
use std::future::Future;

use crate::utils::types::result::Result;

use super::behaviors::BrowserBehavior;
use futures::stream::SplitSink;
use futures::SinkExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tab::Tab;
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio_tungstenite::tungstenite::protocol::Message as WsMessage;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub mod tab;

pub struct Chrome
{
    ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>,
    process: tokio::process::Child,
    tabs: Vec<Tab>,
}
impl BrowserBehavior for Chrome
{
    async fn open(path: &str) -> Result<Self>
    {
        let process = open_process(path);
        let targets = get_devtools_targets().await?;
        let ws_url = &targets[0].web_socket_debugger_url;
        let (mut ws_stream, _) = connect_async(ws_url).await.expect("Failed to connect");
        
        let command = serde_json::json!({
            "id": 1, // Unique ID for the command
            "method": "Page.navigate",
            "params": {
                "url": "https://crates.io/crates/serde"
            }
        });
        ws_stream
        .send(Message::Text(command.to_string()))
        .await
        .expect("Could not send message");

        let mut this = Ok(Self {
            ws_stream,
            tabs: vec![],
            process,
        });


        this.as_mut().unwrap().ws_stream.send(Message::Text(command.to_string()))
        .await
        .expect("Could not send message");

        this
    }
    async fn kill(&mut self)
    {
        _ = self.process.kill().await;
    }
    async fn send_command(&mut self, command: serde_json::Value) -> Result<()>
    {
        todo!()
    }
    fn navigate(&mut self, url: &str) -> impl Future<Output = Result<()>> {
        self.send_command(serde_json::json!({
            "id": 1, // Unique ID for the command
            "method": "Page.navigate",
            "params": {
                "url": url
            }
        }))
    }
}

fn open_process(path: &str) -> tokio::process::Child
{
    let chrome_process = Command::new(path)
        .args([
            "--remote-debugging-port=9222", // Expose DevTools on this port
            "--no-first-run",
            "--no-default-browser-check",
            "--disable-extensions",
            "--disable-gpu",
            "--remote-debugging-address=127.0.0.1", // Make it only accessible locally
            "--disable-background-networking",
            "--disable-sync",
            "--disable-translate",
            "--disable-popup-blocking", // Allow popups for automation
            "--auto-open-devtools-for-tabs",
        ])
        .spawn()
        .expect("Failed to start Chrome");
    chrome_process
}

// Struct to deserialize the response from Chrome's /json endpoint
#[derive(Serialize, Deserialize, Debug)]
struct DevToolsTarget
{
    id: String,
    title: String,
    url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    web_socket_debugger_url: String,
}

// Fetch available targets (pages/tabs) from the DevTools protocol
async fn get_devtools_targets() -> Result<Vec<DevToolsTarget>>
{
    let client = Client::new();
    let res = client
        .get("http://127.0.0.1:9222/json") // CDP endpoint to get available tabs
        .send()
        .await
        .expect("Could not send")
        .json::<Vec<DevToolsTarget>>()
        .await
        .expect("Could not convert to json");

    Ok(res)
}
