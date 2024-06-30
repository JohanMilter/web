use std::path::Path;

use protocol::command::http;

use crate::{
    browser::{
        action_parameters::click::ClickType,
        default::{
            driver::DriverFns,
            element::{Element, ElementFns},
        },
    },
    Error,
};

use super::Edge;

impl<'a> ElementFns<'a, Edge> for Element<'a, Edge>
{
    fn click(&self, click_type: ClickType) -> crate::Result<()>
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        self.parent
            .and_then(|parent| parent.parent)
            .ok_or_else(|| Error::InvalidRefference("Invalid 'Driver<Brave>' refference".into()))?
            .send_command(command)
            .map_err(|_| Error::InvalidRefference("Invalid 'Tab<'a, Brave>' refference".into()))
    }
    fn send_text(&self, text: &str) -> crate::Result<()>
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        self.parent
            .and_then(|parent| parent.parent)
            .ok_or_else(|| Error::InvalidRefference("Invalid 'Driver<Brave>' refference".into()))?
            .send_command(command)
            .map_err(|_| Error::InvalidRefference("Invalid 'Tab<'a, Brave>' refference".into()))
    }
}
