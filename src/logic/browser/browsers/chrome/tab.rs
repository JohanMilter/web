use std::path::Path;

use protocol::command::http;

use crate::{
    logic::browser::default::{
        driver::{CommandResult, DriverFns},
        element::Element,
        tab::{Tab, TabFns},
    },
    utils::types::{error::Error, from::From, result::Result, url::Url},
};

use super::Chrome;

impl<'a> TabFns<'a, Chrome> for Tab<'a, Chrome>
{
    fn get_element(&self, from: From) -> Result<Element<Chrome>>
    {
        let mut command = http::Command::default();
        command.push(http::Element::GET {
            value: String::from(""),
            version: 1.1,
        });

        let _ = self
            .parent
            .ok_or_else(|| Error::InvalidRefference("Invalid '' refference".into()))?
            .send_command(command);

        Ok(Element::<Chrome> {
            parent: Some(self),
            state: std::marker::PhantomData::<Chrome>,
        })
    }
    fn navigate(&self, url: Url) -> Result<CommandResult>
    {
        let mut command = http::Command::default();
        command.push(http::Element::GET {
            value: String::from(""),
            version: 1.1,
        });

        self.parent
            .ok_or_else(|| Error::InvalidRefference("Invalid '' refference".into()))?
            .send_command(command)
    }
}
