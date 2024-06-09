

use crate::{browser::{ChromeTab, Settings}, Command};

#[derive(Clone, PartialEq, Default)]
pub struct ChromeDriver
{
    settings: Settings,
}
//Init
impl ChromeDriver
{
    pub fn open(settings: Settings) -> Self
    {
        Self { settings }
    }
}
//Commands
impl ChromeDriver{
    pub(crate) fn send_command(command: Command){
        
    }
}
//Tabs
impl ChromeDriver {
    pub fn new_tab(&self) -> ChromeTab{
        todo!()
    }
}
