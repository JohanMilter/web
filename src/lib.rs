mod logic;
pub use logic::*;
mod utils;
pub use utils::*;

#[cfg(test)]
mod tests {
    use std::{path::Path, time::Duration};

    use browser::tools::behaviors::{ElementRead, ElementWrite, TabRead};
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
        let (mut _browser, first_tab) = Browser::<Chrome>::open(9222, None).await.unwrap();
        _ = first_tab.navigate("https://www.example.com/").await;
        
        let element = first_tab.get_element(By::XPath("/html/body/div/h1")).await.unwrap();
        let text = element.get_text().await.unwrap();
        println!("text = {}", text["result"]["result"]["value"]);

        let submit_element = first_tab.get_element(By::XPath("/html/body/div/p[2]/a")).await.unwrap();
        _ = submit_element.click().await;
        std::thread::sleep(Duration::from_secs(5));
        _ = first_tab.refresh().await;

        std::thread::sleep(Duration::from_secs(5));
    }

    #[tokio::test]
    async fn multiple_tabs() {
        let (mut browser, first_tab) = Browser::<Chrome>::open(9222, None).await.unwrap();
        let new_tab = browser.new_tab(None).await.unwrap();
        const WAIT: u64 = 2;
        for _ in 0..10 {
            _ = new_tab.navigate("https://www.wikipedia.org/").await;
            _ = first_tab.navigate("https://www.wikipedia.org/").await;
            std::thread::sleep(Duration::from_secs(WAIT));
            _ = new_tab.navigate("https://www.example.com/").await;
            _ = first_tab.navigate("https://www.example.com/").await;
            std::thread::sleep(Duration::from_secs(WAIT));
        }
        std::thread::sleep(Duration::from_secs(5));
    }

    #[tokio::test]
    async fn navigate_back_forward() {
        const WAIT: u64 = 2;
        let (mut browser, first_tab) = Browser::<Chrome>::open(9222, None).await.unwrap();
        _ = first_tab.navigate("https://www.example.com/").await;
        std::thread::sleep(Duration::from_secs(WAIT));
        _ = first_tab.navigate("https://www.wikipedia.org/").await;
        _ = first_tab.back().await;
        /*
        for _ in 0..10 {
            _ = first_tab.back().await;
            std::thread::sleep(Duration::from_secs(WAIT));
            _ = first_tab.forward().await;
            std::thread::sleep(Duration::from_secs(WAIT));
        }
        */
        std::thread::sleep(Duration::from_secs(5));
    }
}
