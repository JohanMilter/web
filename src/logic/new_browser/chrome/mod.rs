use std::path::Path;

use browser_impl::get_devtools_targets;
use tokio::process::Command;
use tokio_tungstenite::connect_async;

use crate::utils::types::result::Result;

use super::{behaviors::DriverBehavior, Tab};
pub mod browser_impl;
pub mod tab_impl;

pub struct Chrome
{
    process: tokio::process::Child, // The browser process
}
impl DriverBehavior for Chrome
{
    async fn open(port: usize, address: &str) -> Result<(Self, Tab<Self>)>
    where
        Self: Sized,
    {
        let process = open_process(
            Path::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe"),
            port,
            address,
        );

        let targets = get_devtools_targets(address, port).await?;
        let first_target = &targets[0];
        let ws_url = &first_target.web_socket_debugger_url;
        let (ws_stream, _) = connect_async(ws_url).await.expect("Failed to connect");

        // TODO Check why the ws_stream doesnt work when moving it
        Ok((
            Self { process },
            Tab::<Chrome> {
                state: std::marker::PhantomData::<Chrome>,
                ws_stream,
            },
        ))
    }
    async fn new_target(&mut self) -> Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
fn open_process(path: &Path, port: usize, address: &str) -> tokio::process::Child
{
    Command::new(path)
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
