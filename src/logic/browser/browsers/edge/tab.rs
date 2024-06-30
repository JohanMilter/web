use std::path::Path;

use protocol::command::http;

use crate::{
    browser::default::{
        driver::DriverFns,
        element::Element,
        tab::{Tab, TabFns},
    },
    Error, From, Url,
};

use super::Edge;

impl<'a> TabFns<'a, Edge> for Tab<'a, Edge>
{
    fn get_element(&self, from: From) -> crate::Result<Element<Edge>>
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

        Ok(Element::<Edge>::builder()
            .parent(Some(self))
            .state(std::marker::PhantomData::<Edge>)
            .build())
    }
    fn navigate(&self, url: Url) -> crate::Result<()>
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
