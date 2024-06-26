mod utils;
pub use utils::*;
mod logic;
pub use logic::*;

#[cfg(test)]
mod tests
{
    use std::path::Path;

    use browser::{action_parameters::click::ClickType, browsers::{brave::Brave, chrome::Chrome, edge::Edge}, default::{driver::{Driver, DriverFns}, element::ElementFns, tab::TabFns}};

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
    fn generate_dir_tree() {
        let connector = chart::file_tree::Connectors::default();
        chart::file_tree::create_dir_tree_file(&connector, &Some(chart::ignore!["target", ".git"]), Path::new(r"P:\Languages\Rust\Libs\web"), Path::new(r"P:\Languages\Rust\Libs\web\docs"))
    }
}
