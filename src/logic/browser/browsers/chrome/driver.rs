use std::{path::Path, process::Command};

use crate::{
    browser::default::{
        driver::{CommandResult, Driver, DriverFns},
        tab::Tab,
    },
    types::result::Result,
};
use protocol::command::http;

use super::Chrome;

impl<'a> DriverFns<'a, Chrome> for Driver<Chrome>
{
    fn new_tab(&'a self) -> Result<Tab<'a, Chrome>>
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        let _ = self.send_command(command);
        Ok(Tab::<'a, Chrome> {
            parent: Some(self),
            state: std::marker::PhantomData::<Chrome>,
        })
    }
    fn open() -> Result<Driver<Chrome>>
    {
        //Make the command Builder to build the commands to be sent to the remote server
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        //Open Chrome in the debbugging mode on port ....
        let child = Command::new(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
            .arg("--remote-debugging-port=9222")
            .spawn()
            .expect("Failed to start Chrome");

        //Make the Driver object to be used as the Client
        let driver = Self {
            state: std::marker::PhantomData::<Chrome>,
            child: Some(child),
        };

        //Send a verification command, to check if it was in fact opened
        let _ = driver.send_command(command);
        Ok(driver)
    }
    fn send_command(&'a self, command: http::Builder) -> Result<CommandResult>
    {
        Ok(CommandResult::Void)
    }
}
