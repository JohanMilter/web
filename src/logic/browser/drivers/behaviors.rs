use std::net::Ipv4Addr;

use crate::{
    logic::browser::{rw::read::DevToolsTarget, tools::tab::Tab, By},
    utils::types::result::Result,
};

pub trait DriverReadBehavior {
    fn open_process(address: Ipv4Addr, port: u16) -> tokio::process::Child;
    fn get_devtools_targets(
        address: Ipv4Addr,
        port: u16,
    ) -> impl std::future::Future<Output = Result<Vec<DevToolsTarget>>> + Send;
}
pub trait DriverWriteBehavior {
    fn navigate(url: &str) -> serde_json::Value;
    fn get_element(by: By, node_id: u32) -> serde_json::Value;
    fn get_document() -> serde_json::Value;
    fn resolve_node(id: u32) -> serde_json::Value;
    fn click_element(object_id: &str) -> serde_json::Value;
    fn new_tab<D: DriverReadBehavior + DriverWriteBehavior>(
        address: Ipv4Addr,
        port: u16,
    ) -> impl std::future::Future<Output = Result<Tab<D>>>;
}
