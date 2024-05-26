use crate::browser::{By, EdgeElement, GetElement, Page};

pub struct EdgePage {}

impl Page for EdgePage
{
    fn kill(self)
    {
        drop(self)
    }
}
impl Drop for EdgePage
{
    fn drop(&mut self)
    {
        println!("Dropped EdgePage");
    }
}

impl GetElement<EdgeElement> for EdgePage
{
    fn get_element(&mut self, by: By) -> Option<EdgeElement>
    {
        todo!()
    }
}
