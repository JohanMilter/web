use crate::browser::{By, Driver, GetElement, Actions, Point, WebElement};

macros::impl_type!(pub struct Brave {
} => Driver {
    async fn open() -> Self {
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
