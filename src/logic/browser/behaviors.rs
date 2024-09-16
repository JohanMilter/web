use crate::utils::types::result::Result;

use super::tab::Tab;

pub trait BrowserBehavior
{
    fn open(path: &str) -> impl std::future::Future<Output = Result<Self>> + Send
    where
        Self: Sized;
    fn kill(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
    fn send_command(&mut self, command: serde_json::Value) -> impl std::future::Future<Output = Result<()>> + Send;
    fn new_tab(&mut self) -> impl std::future::Future<Output = Result<Tab<Self>>> + Send
    where
        Self: Sized;
}
pub trait TabBehavior
{
    fn navigate(&mut self, url: &str) -> impl std::future::Future<Output = Result<()>> + Send;
}
