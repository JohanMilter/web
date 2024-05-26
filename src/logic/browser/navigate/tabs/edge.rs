use crate::browser::{EdgePage, PageNavigation, Tab};

pub struct EdgeTab
{
    pub name: String,
    pub index: usize,
}

impl Tab for EdgeTab
{
    fn kill(self)
    {
        drop(self)
    }
}

impl Drop for EdgeTab
{
    fn drop(&mut self)
    {
        println!("Dropped EdgeTab");
    }
}

impl PageNavigation<EdgePage> for EdgeTab
{
    fn navigate(&mut self, url: &str) -> Option<EdgePage>
    {
        todo!()
    }
}
