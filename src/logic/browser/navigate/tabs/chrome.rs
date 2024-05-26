use crate::browser::{ChromePage, PageNavigation, Tab};

pub struct ChromeTab
{
    pub name: String,
    pub index: usize,
}

impl Tab for ChromeTab
{
    fn kill(self)
    {
        drop(self)
    }
}

impl Drop for ChromeTab
{
    fn drop(&mut self)
    {
        println!("Dropped ChromeTab");
    }
}

impl PageNavigation<ChromePage> for ChromeTab
{
    fn navigate(&mut self, url: &str) -> Option<ChromePage>
    {
        todo!()
    }
}
