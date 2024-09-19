use serde::{Deserialize, Serialize};

use crate::{logic::browser::drivers::behaviors::{DriverRead, DriverWrite}, types::{error::Error, result::Result}};

use super::{behaviors::{ElementRead, ElementWrite, TabWrite}, tab::Tab};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Element<D: DriverRead + DriverWrite>{
    #[serde(skip)]
    pub(crate) state: std::marker::PhantomData<D>,
    pub(crate) object_id: String,
}
impl<D: DriverRead + DriverWrite> ElementRead<D> for Element<D> {
    async fn text(&mut self, tab: &mut Tab<D>) -> Result<serde_json::Value> {
        tab.send_command(D::runtime_call_function_on(D::get_element_innertext(&self.object_id))).await
    }
}
impl<D: DriverRead + DriverWrite> ElementWrite<D> for Element<D> {
    async fn click(&self, tab: &mut Tab<D>) -> Result<serde_json::Value> {
        tab.send_command(D::runtime_call_function_on(D::click_element(&self.object_id))).await
    }
    async fn focus(&self, tab: &mut Tab<D>) -> Result<serde_json::Value> {
        tab.send_command(D::runtime_call_function_on(D::focus(&self.object_id))).await
    }
    async fn set_text(&self, text: &str, tab: &mut Tab<D>) -> Result<serde_json::Value> {
        _ = self.focus(tab).await;
        tab.send_command(D::input_insert_text(D::set_text(text))).await
    }
}