use std::{path::Path, process::Command};

use protocol::command::http;

use crate::browser::default::{driver::{CommandResult, Driver, DriverFns}, tab::Tab};

use super::Edge;

impl<'a> DriverFns<'a, Edge> for Driver<Edge>
{
    fn new_tab(&'a self) -> crate::Result<Tab<'a, Edge>>
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        let _ = self.send_command(command);

        Ok(Tab::<'a, Edge>::builder()
            .parent(Some(self))
            .state(std::marker::PhantomData::<Edge>)
            .build())
    }
    fn open() -> crate::Result<Driver<Edge>>
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        let child = Command::new(r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe")
            .arg("--remote-debugging-port=9222")
            .spawn()
            .expect("Failed to start Edge");

        let driver = Self::builder()
            .state(std::marker::PhantomData::<Edge>)
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
