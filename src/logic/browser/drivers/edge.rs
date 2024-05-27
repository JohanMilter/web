use crate::browser::{EdgeTab, GetTabs, GetDriver, WebDriver};


pub struct EdgeDriver {
}

impl WebDriver for EdgeDriver {
    fn kill(self) {
        drop(self)
    }
}
impl Drop for EdgeDriver {
    fn drop(&mut self) {
        println!("Dropped EdgeDriver");
    }
}

impl GetDriver<EdgeDriver> for EdgeDriver {
    async fn open() -> Option<EdgeDriver> {
        todo!()
    }
} 

impl GetTabs<EdgeTab> for EdgeDriver{
    fn delete_tab(&mut self, name: &str) -> Option<EdgeTab> {
        todo!()
    }
    fn pop_tab(&mut self) -> Option<EdgeTab> {
        todo!()
    }
    fn new_tab(&mut self, name: &str) -> EdgeTab {
        todo!()
    }
}

impl EdgeDriver{

}