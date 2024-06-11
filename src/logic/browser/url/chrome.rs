use crate::{
    browser::{ChromeElement, ChromeTab},
    ElementFrom,
};

pub struct ChromeUrl<'a>
{
    pub(crate) tab_ref: &'a ChromeTab<'a>,
}
impl<'a> ChromeUrl<'a>
{
    pub fn get_element(&'a self, from: ElementFrom) -> ChromeElement<'a>
    {
        ChromeElement { url_ref: self }
    }
}
