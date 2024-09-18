use std::net::Ipv4Addr;

use reqwest::Client;
use tokio::process::Command;

use crate::{
    logic::browser::{rw::read::DevToolsTarget, tools::tab::Tab, By},
    utils::types::result::Result,
};

use super::behaviors::{DriverReadBehavior, DriverWriteBehavior};

pub struct Chrome;
impl DriverReadBehavior for Chrome {
    fn open_process(address: Ipv4Addr, port: u16) -> tokio::process::Child {
        const PATH: &str = r"C:\Program Files\Google\Chrome\Application\chrome.exe";
        Command::new(PATH)
            .args([
                &format!("--remote-debugging-port={port}"), // Expose DevTools on this port
                "--no-first-run",
                "--no-default-browser-check",
                "--disable-extensions",
                "--disable-gpu",
                &format!("--remote-debugging-address={address}"), // Make it only accessible locally
                "--disable-background-networking",
                "--disable-sync",
                "--disable-translate",
                "--disable-popup-blocking", // Allow popups for automation
                "--auto-open-devtools-for-tabs",
            ])
            .spawn()
            .expect("Failed to start Chrome")
    }
    async fn get_devtools_targets(address: Ipv4Addr, port: u16) -> Result<Vec<DevToolsTarget>> {
        Ok(Client::new()
            .get(&format!("http://{address}:{port}/json")) // CDP endpoint to get available tabs
            .send()
            .await
            .expect("Could not send")
            .json::<Vec<DevToolsTarget>>()
            .await
            .expect("Could not convert to json"))
    }
}
impl DriverWriteBehavior for Chrome {
    fn navigate(url: &str) -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "method": "Page.navigate",
            "params": {
                "url": url
            }
        })
    }
    fn get_element(by: By, node_id: u32) -> serde_json::Value {
        match by {
            By::Id(id) => serde_json::json!({
                "id": 3,
                "method": "DOM.querySelector",
                "params": {
                    "nodeId": node_id,
                    "selector": format!("#{id}")
                }
            }),
            By::XPath(xpath) => serde_json::json!({}),
        }
    }
    fn get_document() -> serde_json::Value {
        serde_json::json!({
            "id": 2,
            "method": "DOM.getDocument"
        })
    }
    fn resolve_node(id: u32) -> serde_json::Value {
        serde_json::json!({
            "id": 4,
            "method": "DOM.resolveNode",
            "params": {
                "nodeId": id
            }
        })
    }
    fn click_element(object_id: &str) -> serde_json::Value {
        serde_json::json!({
            "id": 5,
            "method": "Runtime.callFunctionOn",
            "params": {
                "functionDeclaration": "function() { this.click(); }",
                "objectId": object_id,
                "returnByValue": false,
                "awaitPromise": false
            }
        })
    }

    async fn new_tab<D: DriverReadBehavior + DriverWriteBehavior>(
        address: Ipv4Addr,
        port: u16,
    ) -> Result<Tab<D>> {
        let url = format!("http://{address}:{port}/json/new");
        println!("{:?}", url);
        let resp = reqwest::get(&url).await.unwrap();
        println!("{:?}", resp);
        println!("{:?}", url);
        // Deserialize the response into DevToolsTarget
        Tab::init(address, port).await
    }
}
