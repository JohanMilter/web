use std::{future::Future, net::Ipv4Addr};

use crate::{
    logic::browser::tools::tab::Tab, types::by::By,
    utils::types::result::Result,
};

pub trait DriverRead
where
    Self: Sized + DriverWrite, {
    fn get_tabs(connection: (Ipv4Addr, u16)) -> impl Future<Output = Result<Vec<Tab<Self>>>>;
    fn get_element(by: By) -> serde_json::Value;
}
pub trait DriverWrite {
    fn open_process(address: Ipv4Addr, port: u16) -> tokio::process::Child;
    fn navigate(url: &str) -> serde_json::Value;
    fn kill_tab(target_id: &str) -> serde_json::Value;
    fn new_tab() -> serde_json::Value;
    fn click_element(object_id: &str) -> serde_json::Value;
    fn get_text(object_id: &str) -> serde_json::Value;
    fn runtime_evaluate(params: serde_json::Value) -> serde_json::Value;
    fn runtime_call_function_on(params: serde_json::Value) -> serde_json::Value;
    fn focus(object_id: &str) -> serde_json::Value;
    fn input_insert_text(params: serde_json::Value) -> serde_json::Value;
    fn input_dispatch_key_event(params: serde_json::Value) -> serde_json::Value;
    fn set_text(text: &str) -> serde_json::Value;
    fn tab_refresh() -> serde_json::Value;
    fn set_navigation_entry(entry_id: u32) -> serde_json::Value;
    fn get_navigation_history() -> serde_json::Value;
    fn tab_re_attach_to_target(target_id: &str) -> serde_json::Value;
}
