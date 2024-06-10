use std::path::Path;

use protocol::command::http;

use crate::{
    browser::{ChromeTab, Settings},
    Command,
};

#[derive(Clone, PartialEq, Default)]
pub struct ChromeDriver
{
    settings: Settings,
}
//Init
impl ChromeDriver
{
    pub fn open(settings: Settings) -> Self
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new("index.html"),
            version: 1.1,
        });
        let driver = Self { settings };
        driver.send_command(command);
        driver
    }
}
//Commands
impl ChromeDriver
{
    pub(crate) fn send_command(&self, command: http::Builder)
    {
        println!("Sending: \n{}", command);
    }
}
//Tabs
impl ChromeDriver
{
    pub fn new_tab(&self) -> ChromeTab
    {
        ChromeTab {}
    }
}
