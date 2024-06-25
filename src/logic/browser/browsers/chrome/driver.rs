use std::path::Path;

use protocol::command::http;

use crate::browser::default::{driver::Driver, driver::DriverFns, tab::Tab};

use super::Chrome;

impl<'a> DriverFns<'a, Chrome> for Driver<Chrome> {
    fn new_tab(&'a self) -> Tab<'a, Chrome> {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.send_command(command);
        Tab::<'a, Chrome> {
            parent: self,
            state: std::marker::PhantomData::<Chrome>
        }
    }
    fn open() -> Self {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });
        
        let driver = Self {
            state: std::marker::PhantomData::<Chrome>
        };
        driver.send_command(command);
        driver
    }
    fn send_command(&self, command: protocol::command::http::Builder) {
        todo!()
    }
}