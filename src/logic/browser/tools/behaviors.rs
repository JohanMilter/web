use std::future::Future;

use serde::de::DeserializeOwned;

use crate::{browser::drivers::behaviors::{DriverRead, DriverWrite}, types::by::By, utils::types::result::Result};

use super::{element::Element, tab::Tab};


pub trait TabRead<D: DriverRead + DriverWrite>{
    fn get_element(&self, by: By) -> impl Future<Output = Result<Element<D>>>;
    fn send_command<T>(&self, command: serde_json::Value) -> impl Future<Output = Result<T>>
    where
        T: DeserializeOwned;
}
pub trait TabWrite<D: DriverRead+ DriverWrite>{
    fn connect(&mut self) -> impl Future<Output = Result<()>>;
    fn disconnect(&self) -> impl Future<Output = Result<()>>;
    fn navigate(&self, url: &str) -> impl Future<Output = Result<serde_json::Value>>;
    fn kill(&self) -> impl Future<Output = Result<serde_json::Value>>;
}

pub trait ElementRead<D: DriverRead + DriverWrite>{
    fn get_text(&self) -> impl Future<Output = Result<serde_json::Value>>;
    fn send_command<T>(&self, command: serde_json::Value) -> impl Future<Output = Result<T>>
       where
           T: DeserializeOwned;
}
pub trait ElementWrite<D: DriverRead + DriverWrite>{
    fn click(&self) -> impl Future<Output = Result<serde_json::Value>>;
    fn focus(&self) -> impl Future<Output = Result<serde_json::Value>>;
    fn set_text(&self, text: &str) -> impl Future<Output = Result<serde_json::Value>>;
}