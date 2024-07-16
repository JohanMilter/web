use std::path::Path;

use protocol::command::http;

use crate::{
    browser::{
        action_parameters::click::ClickType,
        default::{
            driver::{CommandResult, DriverFns},
            element::{Element, ReadActions, WriteActions},
        },
    }, types::error::Error, types::result::Result
};

use super::Edge;

impl<'a> WriteActions<'a, Edge> for Element<'a, Edge>
{
    fn click(&self, click_type: ClickType) -> Result<CommandResult>
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
    fn send_text(&self, text: &str) -> Result<CommandResult>
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

impl<'a> ReadActions<'a, Edge> for Element<'a, Edge>
{
    fn read(&self, attribute_key: &str) -> Result<CommandResult>
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
