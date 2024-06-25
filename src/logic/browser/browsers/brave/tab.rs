use std::path::Path;

use protocol::command::http;

use crate::{browser::default::{driver::DriverFns, element::Element, tab::{Tab, TabFns}}, From, Url};

use super::Brave;

impl<'a> TabFns<'a, Brave> for Tab<'a, Brave> {
    fn get_element(&self, from: From) -> Element<Brave> {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.parent.send_command(command);

        Element::<Brave> {
            parent: self,
            state: std::marker::PhantomData::<Brave>
        }
    }
    fn navigate(&self, url: Url) {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.parent.send_command(command);
    }
}