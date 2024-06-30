use macros::PubCrate_Builder;

use crate::{browser::action_parameters::click::ClickType, Result};

use super::tab::Tab;

pub trait ElementFns<'a, State> {
    //Actions
    fn click(&self, click_type: ClickType) -> Result<()>;
    fn send_text(&self, text: &str) -> Result<()>;
}
#[derive(PubCrate_Builder, Default, Debug)]
pub struct Element<'a, State>{
    pub(crate) parent: Option<&'a Tab<'a, State>>,
    pub(crate) state: std::marker::PhantomData<State>
}
impl<'a, State> Element<'a, State> {
    
}
