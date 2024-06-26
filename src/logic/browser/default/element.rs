use crate::browser::action_parameters::click::ClickType;

use super::tab::Tab;

pub trait ElementFns<'a, State> {
    //Actions
    fn click(&self, click_type: ClickType);
    fn send_text(&self, text: &str);
}
pub struct Element<'a, State>{
    pub(crate) parent: &'a Tab<'a, State>,
    pub(crate) state: std::marker::PhantomData<State>
}
impl<'a, State> Element<'a, State> {
    
}
