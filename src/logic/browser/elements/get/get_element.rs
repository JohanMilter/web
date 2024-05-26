use crate::browser::WebElement;

pub enum By<'a>
{
    Id(&'a str),
    XPath(&'a str),
    Class(&'a str),
    Name(&'a str),
}

pub trait GetElement<T>
{
    fn get_element(&mut self, by: By) -> Option<T>;
}