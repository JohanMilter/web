use crate::browser::{Actions, WebElement};

pub struct ChromeElement {}

impl WebElement for ChromeElement
{
    fn kill(self)
    {
        drop(self)
    }
}
impl Drop for ChromeElement
{
    fn drop(&mut self)
    {
        println!("Dropped ChromeElement");
    }
}

impl Actions for ChromeElement
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

impl ChromeElement {}
