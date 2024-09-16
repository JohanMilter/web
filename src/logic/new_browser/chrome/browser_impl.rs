use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tokio_tungstenite::connect_async;

use crate::{logic::new_browser::{behaviors::{BrowserBehavior, DriverBehavior, TabBehavior}, Browser}, utils::types::result::Result};

use super::Chrome;

impl<Chrome: DriverBehavior> BrowserBehavior<'_, Chrome> for Browser<Chrome>
{
    async fn new_tab(&mut self) -> Result<crate::logic::new_browser::Tab<Chrome>> where Self: Sized {
        let targets = get_devtools_targets(&self.address, self.port).await?;
        let first_target = targets.last().unwrap();
        let ws_url = &first_target.web_socket_debugger_url;
        let (ws_stream, _) = connect_async(ws_url).await.expect("Failed to connect");


        todo!()
    }
    /*
    async fn get_tab() -> crate::utils::types::result::Result<crate::logic::new_browser::Tab<Chrome>> where Self: Sized {
        todo!()
    }
    */
    async fn open(port: usize) -> Result<(Self, crate::logic::new_browser::Tab<Chrome>)> where Self: Sized {
        let address = "127.0.0.1";
        let (inner, tab) = Chrome::open(port, address).await.unwrap();
        Ok((Self { inner, port, address: address.to_string() }, tab))
    }
}


// Struct to deserialize the response from Chrome's /json endpoint
#[derive(Serialize, Deserialize, Debug)]
pub struct DevToolsTarget
{
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    pub(crate) web_socket_debugger_url: String,
}

// Fetch available targets (pages/tabs) from the DevTools protocol
pub async fn get_devtools_targets(address: &str, port: usize) -> Result<Vec<DevToolsTarget>>
{
    let client = Client::new();
    let res = client
        .get(&format!("http://{address}:{port}/json")) // CDP endpoint to get available tabs
        .send()
        .await
        .expect("Could not send")
        .json::<Vec<DevToolsTarget>>()
        .await
        .expect("Could not convert to json");

    Ok(res)
}
