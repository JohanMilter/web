use std::error::Error;

use headless_chrome::{Browser, LaunchOptions};
use headless_chrome::protocol::cdp::Page;

fn main() -> Result<(), Box<dyn Error>> {
    let options = LaunchOptions::default_builder().headless(false).build()?;
    let browser = Browser::new(options)?;

    let tab = browser.new_tab()?;

    // Navigate to wikipedia
    tab.navigate_to("https://www.wikipedia.org")?;

    // Wait for network/javascript/dom to make the search-box available
    // and click it.
    tab.wait_for_element("input#searchInput")?.click()?;

    // Type in a query and press `Enter`
    tab.type_str("WebKit")?.press_key("Enter")?;

    

    Ok(())
}