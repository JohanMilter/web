use super::{behaviors::{BrowserBehavior, TabBehavior}, chrome::Chrome};

pub struct Tab<'a, T>
where T: BrowserBehavior {
    parent: &'a mut T
}
impl<'a> TabBehavior for Tab<'a, Chrome> {
    async fn navigate(&mut self, url: &str) -> crate::utils::types::result::Result<()> {
        self.parent.send_command(serde_json::json!({
            "id": 1,
            "method": "Page.navigate",
            "params": {
                "url": url
            }
        })).await
    }
}