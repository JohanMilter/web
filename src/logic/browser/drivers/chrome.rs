use std::path::Path;

use protocol::command::http;

use crate::browser::{ChromeTab, Settings};

#[derive(Clone, PartialEq)]
pub struct ChromeDriver<'a>
{
    settings: &'a Settings,
}
//Init
impl<'a> ChromeDriver<'a>
{
    pub fn open(settings: &'a Settings) -> Self
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
impl<'a> ChromeDriver<'a>
{
    pub(crate) fn send_command(&self, command: http::Builder)
    {
        println!("Sending: \n{}", command);
    }
}
//Tabs
impl<'a> ChromeDriver<'a>
{
    pub fn new_tab(&self) -> ChromeTab
    {
        ChromeTab {driver_ref: self}
    }
}
