use std::{path::Path, process::Command};

use protocol::command::http;
use crate::browser::default::{driver::{CommandResult, Driver, DriverFns}, tab::Tab};

use super::Brave;

impl<'a> DriverFns<'a, Brave> for Driver<Brave>
{
    fn new_tab(&'a self) -> crate::Result<Tab<'a, Brave>>
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        let _ = self.send_command(command);

        Ok(Tab::<'a, Brave>::builder()
            .parent(Some(self))
            .state(std::marker::PhantomData::<Brave>)
            .build())
    }
    fn open() -> crate::Result<Driver<Brave>>
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

        let driver = Self::builder()
            .state(std::marker::PhantomData::<Brave>)
            .child(Some(child))
            .build();

        let _ = driver.send_command(command);
        Ok(driver)
    }
    fn send_command(&'a self, command: http::Builder) -> crate::Result<CommandResult>
    {
        Ok(CommandResult::Void)
    }
}
