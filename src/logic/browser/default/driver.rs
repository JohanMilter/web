use std::sync::{Arc, Mutex};

use protocol::command::http;

use super::tab::Tab;

pub trait DriverFns<'a, State> {
    //Init
    fn open() -> Self;

    //Commands
    fn send_command(&'a self, command: http::Builder);

    //Tabs
    fn new_tab(&'a self) -> Tab<'a, State>;
}

//The Driver is the Client that should send and receive http commands
#[derive(Debug)]
pub struct Driver<State>{
    pub(crate) state: std::marker::PhantomData<State>
}
impl<State> Driver<State> {
    
}