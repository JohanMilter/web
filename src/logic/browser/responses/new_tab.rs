use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewTabResponse {
    pub(crate) id: u32,
    pub(crate) result: NewTabResponseResult,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NewTabResponseResult {
    #[serde(rename = "targetId")]
    pub(crate) target_id: String,
}