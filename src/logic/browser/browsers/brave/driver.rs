use std::{path::Path, process::Command};

use crate::{
    browser::default::{
        driver::{CommandResult, Driver, DriverFns},
        tab::Tab,
    },
    types::result::Result,
};
use protocol::command::http;

use super::Brave;

impl<'a> DriverFns<'a, Brave> for Driver<Brave>
{
    fn new_tab(&'a self) -> Result<Tab<'a, Brave>>
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        let _ = self.send_command(command);

        Ok(Tab::<'a, Brave> {
            parent: Some(self),
            state: std::marker::PhantomData::<Brave>,
        })
    }
    fn open() -> Result<Driver<Brave>>
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        let child =
            Command::new(r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe")
                .arg("--remote-debugging-port=9222")
                .spawn()
                .expect("Failed to start Brave");

        let driver = Self {
            state: std::marker::PhantomData::<Brave>,
            child: Some(child),
        };

        let _ = driver.send_command(command);
        Ok(driver)
    }
    fn send_command(&'a self, command: http::Builder) -> Result<CommandResult>
    {
        Ok(CommandResult::Void)
    }
}
