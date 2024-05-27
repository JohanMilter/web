use crate::browser::{GetDriver, WebDriver, ChromeTab, GetTabs};


pub struct ChromeDriver {
}

impl WebDriver for ChromeDriver {
    fn kill(self) {
        drop(self)
    }
}
impl Drop for ChromeDriver {
    fn drop(&mut self) {
        println!("Dropped ChromeDriver");
    }
} 

impl GetDriver<ChromeDriver> for ChromeDriver {
    async fn open() -> Option<ChromeDriver> {
        todo!()
    }
} 

impl GetTabs<ChromeTab> for ChromeDriver{
    fn delete_tab(&mut self, name: &str) -> Option<ChromeTab> {
        todo!()
    }
    fn pop_tab(&mut self) -> Option<ChromeTab> {
        todo!()
    }
    fn new_tab(&mut self, name: &str) -> ChromeTab {
        todo!()
    }
}

impl ChromeDriver{

}