use crate::browser::{BravePage, PageNavigation, Tab};

pub struct BraveTab
{
    pub name: String,
    pub index: usize,
}

impl Tab for BraveTab
{
    fn kill(self)
    {
        drop(self)
    }
}

impl Drop for BraveTab
{
    fn drop(&mut self)
    {
        println!("Dropped BraveTab");
    }
}

impl PageNavigation<BravePage> for BraveTab
{
    fn navigate(&mut self, url: &str) -> Option<BravePage>
    {
        todo!()
    }
}
