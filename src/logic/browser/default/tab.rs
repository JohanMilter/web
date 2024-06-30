use macros::PubCrate_Builder;

use crate::{From, Result, Url};

use super::{driver::Driver, element::Element};

pub trait TabFns<'a, State> {
    //Navigation
    fn navigate(&self, url: Url) -> Result<()>;

    //Elements
    fn get_element(&self, from: From) -> Result<Element<State>>;
}
#[derive(PubCrate_Builder, Default, Debug)]
pub struct Tab<'a, State>{
    pub(crate) parent: Option<&'a Driver<State>>,
    pub(crate) state: std::marker::PhantomData<State>
}
impl<'a, State> Tab<'a, State> {
    
}