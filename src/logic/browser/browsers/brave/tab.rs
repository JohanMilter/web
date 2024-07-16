use std::path::Path;

use protocol::command::http;

use crate::{
    browser::default::{
        driver::{CommandResult, DriverFns},
        element::Element,
        tab::{Tab, TabFns},
    }, types::error::Error, types::from::From, types::result::Result, types::url::Url
};

use super::Brave;

impl<'a> TabFns<'a, Brave> for Tab<'a, Brave>
{
    fn get_element(&self, from: From) -> Result<Element<Brave>>
    {
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
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
        let mut command = http::Builder::default();
        command.push(http::Element::GET {
            value: Path::new(""),
            version: 1.1,
        });

        self.parent
            .ok_or_else(|| Error::InvalidRefference("Invalid '' refference".into()))?
            .send_command(command)
    }
}
