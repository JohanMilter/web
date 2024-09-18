pub mod logic;
pub mod utils;

#[cfg(test)]
mod tests
{
    use std::{path::Path, time::Duration};

    use logic::browser::{drivers::chrome::Chrome, Browser};

    use super::*;

    #[tokio::test]
    async fn test()
    {
        let (_browser, mut first_tab) = Browser::<Chrome>::open(9222).await.unwrap();
        for _ in 0..5 {
            _ = first_tab.navigate("https://www.wikipedia.org/").await.unwrap();
            std::thread::sleep(Duration::from_secs(2));
            _ = first_tab.navigate("https://www.example.com/").await.unwrap();
            std::thread::sleep(Duration::from_secs(2));
        }
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
