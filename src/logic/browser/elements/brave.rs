use crate::browser::{Actions, WebElement};

pub struct BraveElement {}

impl WebElement for BraveElement
{
    fn kill(self)
    {
        drop(self)
    }
}
impl Drop for BraveElement
{
    fn drop(&mut self)
    {
        println!("Dropped BraveElement");
    }
}

impl Actions for BraveElement
{
    fn click(&self, click_type: crate::browser::ClickType) -> bool
    {
        todo!()
    }
    fn text(&self, text: &str) -> bool
    {
        todo!()
    }
}

impl BraveElement {}
