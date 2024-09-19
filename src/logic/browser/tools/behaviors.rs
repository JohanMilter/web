use std::future::Future;

use serde::de::DeserializeOwned;

use crate::{browser::drivers::behaviors::{DriverRead, DriverWrite}, types::by::By, utils::types::result::Result};

use super::{element::Element, tab::Tab};


pub trait TabRead<D: DriverRead + DriverWrite>{
    fn get_element(&mut self, by: By) -> impl Future<Output = Result<Element<D>>>;
}
pub trait TabWrite<D: DriverRead+ DriverWrite>{
    fn send_command<T>(&mut self, command: serde_json::Value) -> impl Future<Output = Result<T>>
    where
        T: DeserializeOwned;
    fn connect(&mut self) -> impl Future<Output = Result<()>>;
    fn disconnect(&mut self) -> impl Future<Output = Result<()>>;
    fn navigate(&mut self, url: &str) -> impl Future<Output = Result<serde_json::Value>>;
    fn kill(&mut self) -> impl Future<Output = Result<serde_json::Value>>;
}

pub trait ElementRead<D: DriverRead + DriverWrite>{
    fn text(&mut self, tab: &mut Tab<D>) -> impl Future<Output = Result<serde_json::Value>>;
}
pub trait ElementWrite<D: DriverRead + DriverWrite>{
    fn click(&self, tab: &mut Tab<D>) -> impl Future<Output = Result<serde_json::Value>>;
    fn focus(&self, tab: &mut Tab<D>) -> impl Future<Output = Result<serde_json::Value>>;
    fn set_text(&self, text: &str, tab: &mut Tab<D>) -> impl Future<Output = Result<serde_json::Value>>;
}