use std::{future::Future, net::Ipv4Addr};

use crate::{logic::browser::tools::tab::Tab, utils::types::result::Result};

pub trait DriverRead {
    fn get_tabs(connection: (Ipv4Addr, u16)) -> impl Future<Output = Result<Vec<Tab<Self>>>>
    where
        Self: Sized + DriverWrite;
}
pub trait DriverWrite {
    fn open_process(address: Ipv4Addr, port: u16) -> tokio::process::Child;
    fn navigate(url: &str) -> serde_json::Value;
    fn kill_tab(target_id: &str) -> serde_json::Value;
    fn new_tab() -> serde_json::Value;
}
