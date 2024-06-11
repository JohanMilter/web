use crate::{
    browser::{BraveDriver, BraveUrl},
    UrlFrom,
};

pub struct BraveTab<'a>
{
    pub(crate) driver_ref: &'a BraveDriver<'a>,
}
impl<'a> BraveTab<'a>
{
    pub fn navigate_url(&'a self, url: UrlFrom) -> BraveUrl<'a>
    {
        BraveUrl { tab_ref: self }
    }
}
