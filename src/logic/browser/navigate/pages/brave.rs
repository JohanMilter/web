use crate::browser::{BraveElement, By, GetElement, Page};

pub struct BravePage {}

impl Page for BravePage
{
    fn kill(self)
    {
        drop(self)
    }
}
impl Drop for BravePage
{
    fn drop(&mut self)
    {
        println!("Dropped BravePage");
    }
}

impl GetElement<BraveElement> for BravePage
{
    fn get_element(&mut self, by: By) -> Option<BraveElement>
    {
        todo!()
    }
}
