use crate::{
    browser::{EdgeDriver, EdgeUrl},
    UrlFrom,
};

pub struct EdgeTab<'a>
{
    pub(crate) driver_ref: &'a EdgeDriver<'a>,
}
impl<'a> EdgeTab<'a>
{
    pub fn navigate_url(&'a self, url: UrlFrom) -> EdgeUrl<'a>
    {
        EdgeUrl { tab_ref: self }
    }
}
