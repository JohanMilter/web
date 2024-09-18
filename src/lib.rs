mod logic;
pub use logic::*;
mod utils;
pub use utils::*;

#[cfg(test)]
mod tests {
    use std::{path::Path, time::Duration};

    use logic::browser::{
        behaviors::{BrowserRead, BrowserWrite},
        drivers::chrome::Chrome,
        tools::{behaviors::TabWrite, tab::TabOptions},
        Browser, BrowserOptions,
    };

    use super::*;

    #[tokio::test]
    async fn test() {

        let (mut browser, mut first_tab) = Browser::<Chrome>::open(9222, None).await.unwrap();
        _ = first_tab.navigate("https://www.example.com/").await;

        let mut new_tab1 = browser.new_tab(None).await.unwrap();
        _ = new_tab1.navigate("https://www.wikipedia.org/").await;

        let mut new_tab2 = browser.new_tab(None).await.unwrap();
        _ = new_tab2.navigate("https://www.netflix.com/").await;

        std::thread::sleep(Duration::from_secs(3));
    }

    #[test]
    fn generate_dir_tree() {
        let connector = chart::file_tree::Connectors::default();
        chart::file_tree::create_dir_tree_file(
            &connector,
            &Some(chart::ignore!["target", ".git"]),
            Path::new(r"P:\Languages\Rust\Libs\web"),
            Path::new(r"P:\Languages\Rust\Libs\web\docs"),
        )
    }
}
