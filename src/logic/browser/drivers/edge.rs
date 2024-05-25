use crate::browser::{By, Driver, Actions, Point, WebElement};

use super::GetElement;

macros::impl_type!(pub struct Edge {
} => Driver {
    async fn open() -> Self{
        todo!()
    }
} => Actions {
    fn click_at(&self, element: &Box<dyn WebElement>) -> bool {
        todo!()
    }
    fn type_at(&self, element: &Box<dyn WebElement>, text: &str) -> bool{
        todo!()
    }
    fn set_cursor_at(&self, point: &Point) -> bool {
        todo!()
    }
} => GetElement {
    fn get_element(&self, by: By) -> Box<dyn WebElement> {
        todo!()
    }
} => {

});