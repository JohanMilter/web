use std::future::Future;

use serde::de::DeserializeOwned;

use crate::utils::types::result::Result;


pub trait TabRead{

}
pub trait TabWrite{
    fn send_command<T>(&mut self, command: serde_json::Value) -> impl Future<Output = Result<T>>
    where
        T: DeserializeOwned;
    fn connect(&mut self) -> impl Future<Output = Result<()>>;
    fn disconnect(&mut self) -> impl Future<Output = Result<()>>;
    fn navigate(&mut self, url: &str) -> impl Future<Output = Result<serde_json::Value>>;
    fn kill(&mut self) -> impl Future<Output = Result<serde_json::Value>>;
}

pub trait ElementRead{
    
}
pub trait ElementWrite{

}