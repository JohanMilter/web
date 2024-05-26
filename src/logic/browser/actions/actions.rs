use crate::browser::WebElement;

pub struct Point
{
    pub x: isize,
    pub y: isize,
}
pub enum ClickType{
    Left,
    Right,
    Middle,
}

pub trait Actions
{
    fn click(&self, click_type: ClickType) -> bool;
    fn text(&self, text: &str) -> bool;
}

