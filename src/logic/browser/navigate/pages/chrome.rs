use crate::browser::{By, ChromeElement, GetElement, Page};

pub struct ChromePage {}

impl Page for ChromePage
{
    fn kill(self)
    {
        drop(self)
    }
}
impl Drop for ChromePage
{
    fn drop(&mut self)
    {
        println!("Dropped ChromePage");
    }
}

impl GetElement<ChromeElement> for ChromePage
{
    fn get_element(&mut self, by: By) -> Option<ChromeElement>
    {
        todo!()
    }
}
