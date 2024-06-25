use std::path::Path;

use protocol::command::http;

use crate::{browser::default::{driver::DriverFns, element::Element, tab::{Tab, TabFns}}, From, Url};

use super::Edge;

impl<'a> TabFns<'a, Edge> for Tab<'a, Edge> {
    fn get_element(&self, from: From) -> Element<Edge> {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.parent.send_command(command);

        Element::<Edge> {
            parent: self,
            state: std::marker::PhantomData::<Edge>
        }
    }
    fn navigate(&self, url: Url) {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.parent.send_command(command);
    }
}