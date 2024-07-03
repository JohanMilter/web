use std::{thread, time::Duration};

use headless_chrome::{Browser, LaunchOptions};


fn main() {
    let options = LaunchOptions::default_builder().headless(false).build().unwrap();
    let browser = Browser::new(options).unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://www.wikipedia.org").unwrap();
    thread::sleep(Duration::from_secs(30));
}