use crate::{logic::browser::action_parameters::click::ClickType, utils::types::result::Result};

use super::{driver::CommandResult, tab::Tab};

pub trait WriteActions<'a, State>
{
    //Actions
    fn click(&self, click_type: ClickType) -> Result<CommandResult>;
    fn send_text(&self, text: &str) -> Result<CommandResult>;
}
pub trait ReadActions<'a, State>
{
    //Actions
    fn read(&self, attribute_key: &str) -> Result<CommandResult>;
}
#[derive(Default, Debug)]
pub struct Element<'a, State>
{
    pub(crate) parent: Option<&'a Tab<'a, State>>,
    pub(crate) state: std::marker::PhantomData<State>,
}
impl<'a, State> Element<'a, State> {}
