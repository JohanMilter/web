use futures::SinkExt;
use tokio_tungstenite::tungstenite::Message;

use crate::logic::new_browser::{behaviors::{BrowserBehavior, DriverBehavior, TabBehavior}, Tab};

use super::Chrome;

impl<Chrome: DriverBehavior> TabBehavior for Tab<Chrome>{
    async fn navigate(&mut self, url: &str) -> crate::utils::types::result::Result<()>{
        let command = serde_json::json!({
            "id": 1,
            "method": "Page.navigate",
            "params": {
                "url": url
            }
        });
        self.ws_stream.send(Message::Text(command.to_string())).await?;
        Ok(())
    }
    async fn kill(&mut self) -> crate::utils::types::result::Result<()> {
        todo!()
    }
}