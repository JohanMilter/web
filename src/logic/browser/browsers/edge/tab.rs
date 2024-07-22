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

use super::Edge;

impl<'a> TabFns<'a, Edge> for Tab<'a, Edge>
{
    fn get_element(&self, from: From) -> Result<Element<Edge>>
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

        Ok(Element::<Edge>{
            parent: Some(self),
            state: std::marker::PhantomData::<Edge>,
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
