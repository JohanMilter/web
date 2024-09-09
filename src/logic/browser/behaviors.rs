use crate::utils::types::result::Result;

pub trait BrowserBehavior
{
    async fn open(path: &str) -> Result<Self>
    where
        Self: Sized;
    async fn kill(&mut self);
    async fn send_command(&mut self, command: serde_json::Value) -> Result<()>;
    async fn navigate(&mut self, url: &str) -> Result<()>;
}
pub trait TabBehavior
{
    fn navigate(url: &str) -> Result<()>;
}
