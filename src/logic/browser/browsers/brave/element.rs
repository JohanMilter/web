use std::path::Path;

use protocol::command::http;

use crate::{
    browser::{
        action_parameters::click::ClickType,
        default::{
            driver::{CommandResult, DriverFns},
            element::{Element, ReadActions, WriteActions},
        },
    },
    Error,
};

use super::Brave;

impl<'a> WriteActions<'a, Brave> for Element<'a, Brave>
{
    fn click(&self, click_type: ClickType) -> crate::Result<CommandResult>
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
    }
    fn send_text(&self, text: &str) -> crate::Result<CommandResult>
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
    }
}
impl<'a> ReadActions<'a, Brave> for Element<'a, Brave>
{
    fn read(&self, attribute_key: &str) -> crate::Result<CommandResult>
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
    }
}
