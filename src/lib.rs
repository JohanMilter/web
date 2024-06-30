mod utils;
pub use utils::*;
mod logic;
pub use logic::*;

#[cfg(test)]
mod tests
{
    use std::{path::Path, thread, time::Duration};

    use browser::{browsers::{brave::Brave, chrome::Chrome, edge::Edge}, default::{driver::{Driver, DriverFns}, element::ElementFns, tab::TabFns}};

    use super::*;

    #[test]
    fn test() -> Result<()>
    {
        let driver = Driver::<Brave>::open()?;
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn generate_dir_tree() {
        let connector = chart::file_tree::Connectors::default();
        chart::file_tree::create_dir_tree_file(&connector, &Some(chart::ignore!["target", ".git"]), Path::new(r"P:\Languages\Rust\Libs\web"), Path::new(r"P:\Languages\Rust\Libs\web\docs"))
    }
}
