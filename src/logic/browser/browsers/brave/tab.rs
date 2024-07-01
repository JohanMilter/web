use std::path::Path;

use protocol::command::http;

use crate::{
    browser::default::{
        driver::{CommandResult, DriverFns},
        element::Element,
        tab::{Tab, TabFns},
    },
    Error, From, Url,
};

use super::Brave;

impl<'a> TabFns<'a, Brave> for Tab<'a, Brave>
{
    fn get_element(&self, from: From) -> crate::Result<Element<Brave>>
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

        Ok(Element::<Brave>::builder()
            .parent(Some(self))
            .state(std::marker::PhantomData::<Brave>)
            .build())
    }
    fn navigate(&self, url: Url) -> crate::Result<CommandResult>
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
