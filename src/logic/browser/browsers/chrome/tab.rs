use std::path::Path;

use protocol::command::http;

use crate::{browser::default::{driver::DriverFns, element::Element, tab::{Tab, TabFns}}, From, Url};

use super::Chrome;

impl<'a> TabFns<'a, Chrome> for Tab<'a, Chrome> {
    fn get_element(&self, from: From) -> Element<Chrome> {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.parent.send_command(command);

        Element::<Chrome> {
            parent: self,
            state: std::marker::PhantomData::<Chrome>
        }
    }
    fn navigate(&self, url: Url) {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.parent.send_command(command);
    }
}