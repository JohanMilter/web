use crate::browser::WebElement;

pub struct Point
{
    pub x: isize,
    pub y: isize,
}

pub trait Actions
{
    fn click_at(&self, element: &Box<dyn WebElement>) -> bool;
    fn type_at(&self, element: &Box<dyn WebElement>, text: &str) -> bool;
    fn set_cursor_at(&self, point: &Point) -> bool;
}

