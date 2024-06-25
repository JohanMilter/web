use std::path::Path;

use protocol::command::http;

use crate::browser::{actions::click::ClickType, default::{driver::DriverFns, element::{Element, ElementFns}}};

use super::Chrome;

impl<'a> ElementFns<'a, Chrome> for Element<'a, Chrome> {
    fn click(&self, click_type: ClickType) {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.parent.parent.send_command(command);
    }
    fn send_text(&self, text: &str) {
        let mut command = http::Builder::default();
        command.push(http::Element::GET { value: Path::new(""), version: 1.1 });

        self.parent.parent.send_command(command);
    }
}