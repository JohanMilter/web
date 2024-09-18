use std::future::Future;

use serde::de::DeserializeOwned;

use crate::utils::types::result::Result;

use super::{drivers::behaviors::{DriverRead, DriverWrite}, tools::tab::{Tab, TabOptions}, BrowserOptions};

pub trait BrowserRead<D: DriverRead + DriverWrite> {
    fn get_tabs(&self) -> impl Future<Output = Result<Vec<Tab<D>>>>
    where 
        Self: Sized;
}
pub trait BrowserWrite<D: DriverRead + DriverWrite> {
    fn send_command<T>(&mut self, command: serde_json::Value) -> impl Future<Output = Result<T>>
    where
        T: DeserializeOwned;
    fn open(port: u16, options: Option<BrowserOptions>) -> impl Future<Output = Result<(Self, Tab<D>)>>
    where
        Self: Sized;
    fn connect(&mut self) -> impl Future<Output = Result<()>>;
    fn disconnect(&mut self) -> impl Future<Output = Result<()>>;
    fn new_tab(&mut self, options: Option<TabOptions>) -> impl Future<Output = Result<Tab<D>>>;
}
