mod components;
macros::import!(logic > pub [*]);

#[cfg(test)]
mod tests
{

    use super::browser::{
        Actions, By, ClickType, GetDriver, GetElement, GetTabs, PageNavigation, Page, Tab, WebDriver,
    };
    use super::*;

    #[tokio::test]
    async fn test()
    {
        //Load the browser
        let mut browser = browser::BraveDriver::open().await.unwrap();

        //Get the tab
        let mut tab = browser.new_tab("searching hello");

        //Get the page
        let mut page = tab.navigate("https://www.google.com/").unwrap();

        //Get the element
        let element = page.get_element(By::Id("")).unwrap();

        //Click the element
        element.click(ClickType::Left);
        
        
    }

    #[test]
    fn generate_dir_tree() {}
}
