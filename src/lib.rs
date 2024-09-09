pub mod logic;
pub mod utils;

#[cfg(test)]
mod tests
{
    use std::path::Path;

    use logic::browser::{behaviors::BrowserBehavior, chrome::Chrome, Browser};

    use super::*;

    #[tokio::test]
    async fn test()
    {
        let mut browser = Browser::<Chrome>::open().await;
        _ = browser.kill();
    }

    #[test]
    fn generate_dir_tree()
    {
        let connector = chart::file_tree::Connectors::default();
        chart::file_tree::create_dir_tree_file(
            &connector,
            &Some(chart::ignore!["target", ".git"]),
            Path::new(r"P:\Languages\Rust\Libs\web"),
            Path::new(r"P:\Languages\Rust\Libs\web\docs"),
        )
    }
}
