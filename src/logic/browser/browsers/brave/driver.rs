use std::path::Path;

use protocol::command::http;

use crate::browser::default::{driver::Driver, driver::DriverFns, tab::Tab};

use super::Brave;

impl<'a> DriverFns<'a, Brave> for Driver<Brave> {
    fn new_tab(&'a self) -> Tab<'a, Brave> {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.send_command(command);
        Tab::<'a, Brave> {
            parent: self,
            state: std::marker::PhantomData::<Brave>
        }
    }
    fn open() -> Self {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });
        
        let driver = Self {
            state: std::marker::PhantomData::<Brave>
        };
        driver.send_command(command);
        driver
    }
    fn send_command(&self, command: protocol::command::http::Builder) {
        todo!()
    }
}