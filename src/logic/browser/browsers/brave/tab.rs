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

use super::Brave;

impl<'a> TabFns<'a, Brave> for Tab<'a, Brave>
{
    fn get_element(&self, from: From) -> Result<Element<Brave>>
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

        Ok(Element::<Brave> {
            parent: Some(self),
            state: std::marker::PhantomData::<Brave>,
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
