use crate::utils::types::{from::From, result::Result, url::Url};

use super::{
    driver::{CommandResult, Driver},
    element::Element,
};

pub trait TabFns<'a, State>
{
    //Navigation
    fn navigate(&self, url: Url) -> Result<CommandResult>;

    //Elements
    fn get_element(&self, from: From) -> Result<Element<State>>;
}
#[derive(Default, Debug)]
pub struct Tab<'a, State>
{
    pub(crate) parent: Option<&'a Driver<State>>,
    pub(crate) state: std::marker::PhantomData<State>,
}
impl<'a, State> Tab<'a, State> {}
