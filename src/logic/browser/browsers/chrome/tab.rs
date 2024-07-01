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

use super::Chrome;

impl<'a> TabFns<'a, Chrome> for Tab<'a, Chrome>
{
    fn get_element(&self, from: From) -> crate::Result<Element<Chrome>>
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

        Ok(Element::<Chrome>::builder()
            .parent(Some(self))
            .state(std::marker::PhantomData::<Chrome>)
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
