use crate::{
    browser::{ChromeDriver, ChromeUrl},
    UrlFrom,
};

pub struct ChromeTab<'a>
{
    pub(crate) driver_ref: &'a ChromeDriver<'a>,
}
impl<'a> ChromeTab<'a>
{
    pub fn navigate_url(&'a self, url: UrlFrom) -> ChromeUrl<'a>
    {
        ChromeUrl { tab_ref: self }
    }
}
