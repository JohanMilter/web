use crate::logic::browser::behaviors::DriverBehavior;

pub struct Chrome;
impl DriverBehavior for Chrome {
    fn navigate(url: &str) -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "method": "Page.navigate",
            "params": {
                "url": url
            }
        })
    }
}
