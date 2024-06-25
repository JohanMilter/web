mod utils;
pub use utils::*;
mod logic;
pub use logic::*;

#[cfg(test)]
mod tests
{
    use browser::{actions::click::ClickType, browsers::{brave::Brave, chrome::Chrome, edge::Edge}, default::{driver::{Driver, DriverFns}, element::ElementFns, tab::TabFns}};

    use super::*;

    #[test]
    fn test()
    {
        let driver = Driver::<Chrome>::open();
        let tab = driver.new_tab();
        let element = tab.get_element(From::Id(""));
        element.click(ClickType::Left);
        
    }

    #[test]
    fn generate_dir_tree() {}
}
