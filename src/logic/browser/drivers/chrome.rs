use std::sync::atomic::{AtomicUsize, Ordering};

use reqwest::Client;
use tokio::process::Command;

use crate::{
    logic::browser::tools::{behaviors::TabWrite, tab::Tab},
    utils::types::result::Result,
};

use super::behaviors::{DriverRead, DriverWrite};

static mut CURRENT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Chrome;
impl DriverRead for Chrome {
    async fn get_tabs(connection: (std::net::Ipv4Addr, u16)) -> Result<Vec<Tab<Self>>>
    where
        Self: Sized + DriverWrite, {
            Ok(Client::new()
            .get(&format!("http://{}:{}/json/list", connection.0, connection.1)) // CDP endpoint to get available tabs
            .send()
            .await
            .expect("Could not send")
            .json::<Vec<Tab<Self>>>()
            .await
            .expect("Could not convert to json"))
    }
}
impl DriverWrite for Chrome {
    fn open_process(address: std::net::Ipv4Addr, port: u16) -> tokio::process::Child {
        unsafe {
            CURRENT_ID.fetch_add(1, Ordering::SeqCst);
        }
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
    fn navigate(url: &str) -> serde_json::Value {
        let current_id;
        unsafe {
            current_id = CURRENT_ID.fetch_add(1, Ordering::SeqCst);
        }
        serde_json::json!({
            "id": current_id,
            "method": "Page.navigate",
            "params": {
                "url": url
            }
        })
    }
    fn kill_tab(target_id: &str) -> serde_json::Value {
        let current_id;
        unsafe {
            current_id = CURRENT_ID.fetch_add(1, Ordering::SeqCst);
        }
        serde_json::json!({
            "id": current_id,
            "method": "Target.closeTarget",
            "params": {
                "targetId": target_id
            }
        })
    }
    fn new_tab() -> serde_json::Value {
        let current_id;
        unsafe {
            current_id = CURRENT_ID.fetch_add(1, Ordering::SeqCst);
        }
        serde_json::json!({
            "id": current_id,
            "method": "Target.createTarget",
            "params": {
                "url": "about:blank"
            }
        })
    }
}
