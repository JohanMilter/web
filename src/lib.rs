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
    use scraper::parse::html::Html;
    use types::by::By;

    use super::*;

    #[tokio::test]
    async fn test() {
        let (mut _browser, first_tab) = Browser::<Chrome>::open(9222, None).await.unwrap();
        _ = first_tab.navigate("https://www.example.com/").await;

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
        let guidelines = get_guidelines("https://www.example.com/");
        let (browser, first_tab) = Browser::<Chrome>::open(9222, None).await.unwrap();
        _ = first_tab.navigate("https://www.example.com/").await;
        std::thread::sleep(Duration::from_secs(WAIT));
        _ = first_tab.navigate("https://www.wikipedia.org/").await;
        for _ in 0..10 {
            _ = first_tab.back(1).await;
            std::thread::sleep(Duration::from_secs(WAIT));
            _ = first_tab.forward(1).await;
            std::thread::sleep(Duration::from_secs(WAIT));
        }
        std::thread::sleep(Duration::from_secs(5));
    }



    #[tokio::test]
    async fn parse_html(){
        let html = r#"
        <html><head>
    <title>Example Domain</title>

    <meta charset="utf-8">
    <meta http-equiv="Content-type" content="text/html; charset=utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style type="text/css">
    body {
        background-color: #f0f0f2;
        margin: 0;
        padding: 0;
        font-family: -apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", "Open Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;
        
    }
    div {
        width: 600px;
        margin: 5em auto;
        padding: 2em;
        background-color: #fdfdff;
        border-radius: 0.5em;
        box-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02);
    }
    a:link, a:visited {
        color: #38488f;
        text-decoration: none;
    }
    @media (max-width: 700px) {
        div {
            margin: 0 auto;
            width: auto;
        }
    }
    </style>    
</head>

<body>
<div>
    <h1>Example Domain</h1>
    <p>This domain is for use in illustrative examples in documents. You may use this
    domain in literature without prior coordination or asking for permission.</p>
    <p><a href="https://www.iana.org/domains/example">More information...</a></p>
</div>


</body></html>
        "#.to_string();
        let html = Html::from(html);
        let node = html.parse(None);
        
    }





}
