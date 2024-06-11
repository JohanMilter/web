use crate::{
    browser::{EdgeElement, EdgeTab},
    ElementFrom,
};

pub struct EdgeUrl<'a>
{
    pub(crate) tab_ref: &'a EdgeTab<'a>,
}
impl<'a> EdgeUrl<'a>
{
    pub fn get_element(&'a self, from: ElementFrom) -> EdgeElement<'a>
    {
        EdgeElement { url_ref: self }
    }
}
