pub mod logic;
pub mod utils;

#[cfg(test)]
mod tests
{
    use std::path::Path;

    use logic::new_browser::{behaviors::{BrowserBehavior, TabBehavior}, chrome::Chrome, Browser};

    use super::*;

    #[tokio::test]
    async fn test()
    {
        let (mut browser, mut tab) = Browser::<Chrome>::open(9222).await.unwrap();
        _ = tab.navigate("https://www.netflix.com/browse");
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
