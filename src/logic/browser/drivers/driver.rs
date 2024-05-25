use crate::browser::WebElement;

pub trait Driver
{
    async fn open() -> Self;
}

pub enum By<'a>
{
    Id(&'a str),
    XPath(&'a str),
    Class(&'a str),
    Name(&'a str),
}

pub trait GetElement
{
    fn get_element(&self, by: By) -> Box<dyn WebElement>;
}