mod logic;
pub use logic::*;
mod utils;
pub use utils::*;

#[cfg(test)]
mod tests {
    use std::{path::Path, time::Duration};

    use browser::tools::behaviors::{ElementWrite, TabRead};
    use logic::browser::{
        behaviors::{BrowserRead, BrowserWrite},
        drivers::chrome::Chrome,
        tools::{behaviors::TabWrite, tab::TabOptions},
        Browser, BrowserOptions,
    };
    use types::by::By;

    use super::*;

    #[tokio::test]
    async fn test() {
        let (mut _browser, mut first_tab) = Browser::<Chrome>::open(9222, None).await.unwrap();
        _ = first_tab.navigate("https://www.wikipedia.org/").await;
        let element = first_tab.get_element(By::Id("searchInput")).await.unwrap();
        _ = element.set_text("Hello", &mut first_tab).await;
        let submit_element = first_tab.get_element(By::XPath("/html/body/main/div[2]/form/fieldset/button")).await.unwrap();
        _ = submit_element.click(&mut first_tab).await;

        std::thread::sleep(Duration::from_secs(5));
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
