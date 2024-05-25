use std::collections::HashMap;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::browser::{Actions, By, Driver, GetElement, Point, WebElement};

#[derive(Serialize)]
struct Capabilities
{
    alwaysMatch: HashMap<String, String>,
}

#[derive(Serialize)]
struct SessionRequest
{
    capabilities: Capabilities,
}

#[derive(Deserialize)]
struct SessionResponse
{
    sessionId: String,
}
async fn create_session(client: &Client) -> Result<String, reqwest::Error>
{
    let mut always_match = HashMap::new();
    always_match.insert("browserName".to_string(), "chrome".to_string());

    let session_request = SessionRequest {
        capabilities: Capabilities {
            alwaysMatch: always_match,
        },
    };

    let response = client
        .post("http://localhost:9515/session")
        .json(&session_request)
        .send()
        .await?
        .json::<SessionResponse>()
        .await?;

    Ok(response.sessionId)
}

macros::impl_type!(pub struct Chrome {
} => Driver {
    async fn open() -> Self{

        let client = Client::new();

        match create_session(&client).await {
            Ok(session_id) => {
                println!("Session ID: {}", session_id);
                // You can add more interactions with the browser here
            }
            Err(err) => {
                eprintln!("Error creating session: {}", err);
            }
        }
        todo!()
    }
} => Actions {
    fn click_at(&self, element: &Box<dyn WebElement>) -> bool {
        todo!()
    }
    fn type_at(&self, element: &Box<dyn WebElement>, text: &str) -> bool{
        todo!()
    }
    fn set_cursor_at(&self, point: &Point) -> bool {
        todo!()
    }
} => GetElement {
    fn get_element(&self, by: By) -> Box<dyn WebElement> {
        todo!()
    }
} => {

});
