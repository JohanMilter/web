use crate::browser::{Actions, WebElement};

pub struct EdgeElement {}

impl WebElement for EdgeElement
{
    fn kill(self)
    {
        drop(self)
    }
}
impl Drop for EdgeElement
{
    fn drop(&mut self)
    {
        println!("Dropped EdgeElement");
    }
}

impl Actions for EdgeElement
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

impl EdgeElement {}
