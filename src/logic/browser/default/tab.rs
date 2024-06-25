use crate::{From, Url};

use super::{driver::Driver, element::Element};

pub trait TabFns<'a, State> {
    //Navigation
    fn navigate(&self, url: Url);

    //Elements
    fn get_element(&self, from: From) -> Element<State>;
}
pub struct Tab<'a, State>{
    pub(crate) parent: &'a Driver<State>,
    pub(crate) state: std::marker::PhantomData<State>
}
impl<'a, State> Tab<'a, State> {
    
}