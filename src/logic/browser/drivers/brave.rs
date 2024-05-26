use crate::browser::{BraveTab, GetTabs, GetDriver, WebDriver};


pub struct BraveDriver {
}

impl WebDriver for BraveDriver {
    fn kill(self) {
        drop(self)
    }
}
impl Drop for BraveDriver {
    fn drop(&mut self) {
        println!("Dropped BraveDriver");
    }
} 


impl GetDriver<BraveDriver> for BraveDriver {
    async fn open() -> Option<BraveDriver> {
        todo!()
    }
} 

impl GetTabs<BraveTab> for BraveDriver{
    fn delete_tab(&mut self, name: &str) -> Option<BraveTab> {
        todo!()
    }
    fn pop_tab(&mut self) -> Option<BraveTab> {
        todo!()
    }
    fn goto_tab(&self, name: &str) -> Option<BraveTab> {
        todo!()
    }
    fn new_tab(&mut self, name: &str) -> BraveTab {
        todo!()
    }
}

impl BraveDriver{

}