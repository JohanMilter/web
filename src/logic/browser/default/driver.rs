use macros::PubCrate_Builder;
use protocol::command::http;

use crate::Result;

use super::tab::Tab;

pub trait DriverFns<'a, State>
{
    //Init
    fn open() -> Result<Driver<State>>;

    //Commands
    fn send_command(&'a self, command: http::Builder) -> Result<()>;

    //Tabs
    fn new_tab(&'a self) -> Result<Tab<'a, State>>;
}

//The Driver is the Client that should send and receive http commands
#[derive(PubCrate_Builder, Default, Debug)]
pub struct Driver<State>
{
    pub(crate) state: std::marker::PhantomData<State>,
    pub(crate) child: Option<std::process::Child>,
}
impl<State> Drop for Driver<State>
{
    fn drop(&mut self)
    {
        println!("[Kill] >> Child");
        self.child
            .as_mut()
            .expect("No child to drop")
            .kill()
            .expect("Could not kill child");
    }
}
impl<State> Driver<State> {}
