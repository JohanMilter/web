use std::sync::atomic::{AtomicUsize, Ordering};

use reqwest::Client;
use tokio::process::Command;

use crate::{
    functions::escape_js_string, logic::browser::tools::tab::Tab, types::by::By,
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
            .get(&format!(
                "http://{}:{}/json/list",
                connection.0, connection.1
            )) // CDP endpoint to get available tabs
            .send()
            .await
            .expect("Could not send")
            .json::<Vec<Tab<Self>>>()
            .await
            .expect("Could not convert to json"))
    }
    fn get_element(by: By<'_>) -> serde_json::Value {
        let expression = match by {
            By::Id(id) => format!("document.querySelector('#{id}')"),
            By::XPath(xpath) => {
                format!(
                    "document.evaluate(\"{}\", document, null, XPathResult.FIRST_ORDERED_NODE_TYPE, null).singleNodeValue",
                    escape_js_string(xpath)
                )
            }
            // Handle other variants as needed
        };
        serde_json::json!({
            "expression": expression,
            "objectGroup": "console",
            "includeCommandLineAPI": true,
            "returnByValue": false,
            "awaitPromise": false,
        })
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
    fn click_element(object_id: &str) -> serde_json::Value {
        serde_json::json!({
            "objectId": object_id,
            "functionDeclaration": "function() { this.click(); }",
            "returnByValue": false,
            "awaitPromise": false,
        })
    }
    fn get_element_innertext(object_id: &str) -> serde_json::Value {
        serde_json::json!({
            "objectId": object_id,
            "functionDeclaration": "function() { return this.innerText; }",
            "returnByValue": true,
            "awaitPromise": false,
        })
    }
    fn runtime_evaluate(params: serde_json::Value) -> serde_json::Value {
        let current_id;
        unsafe {
            current_id = CURRENT_ID.fetch_add(1, Ordering::SeqCst);
        }
        serde_json::json!({
            "id": current_id,
            "method": "Runtime.evaluate",
            "params": params,
        })
    }
    fn runtime_call_function_on(params: serde_json::Value) -> serde_json::Value {
        let current_id;
        unsafe {
            current_id = CURRENT_ID.fetch_add(1, Ordering::SeqCst);
        }
        serde_json::json!({
            "id": current_id,
            "method": "Runtime.callFunctionOn",
            "params": params,
        })
    }
    fn focus(object_id: &str) -> serde_json::Value {
        serde_json::json!({
            "objectId": object_id,
            "functionDeclaration": "function() { this.focus(); }",
            "returnByValue": false,
            "awaitPromise": false,
        })
    }
    fn input_insert_text(params: serde_json::Value) -> serde_json::Value {
        let current_id;
        unsafe {
            current_id = CURRENT_ID.fetch_add(1, Ordering::SeqCst);
        }
        serde_json::json!({
            "id": current_id,
            "method": "Input.insertText",
            "params": params,
        })
    }
    fn set_text(text: &str) -> serde_json::Value {
        serde_json::json!({
            "text": text,
        })
    }

    fn input_dispatch_key_event(params: serde_json::Value) -> serde_json::Value {
        let current_id;
        unsafe {
            current_id = CURRENT_ID.fetch_add(1, Ordering::SeqCst);
        }
        serde_json::json!({
            "id": current_id,
            "method": "Input.dispatchKeyEvent",
            "params": params,
        })
    }
}
