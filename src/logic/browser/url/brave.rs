use crate::{
    browser::{BraveElement, BraveTab},
    ElementFrom,
};

pub struct BraveUrl<'a>
{
    pub(crate) tab_ref: &'a BraveTab<'a>,
}
impl<'a> BraveUrl<'a>
{
    pub fn get_element(&'a self, from: ElementFrom) -> BraveElement<'a>
    {
        BraveElement { url_ref: self }
    }
}
