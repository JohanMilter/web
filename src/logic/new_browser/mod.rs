use std::ops::{Deref, DerefMut};

use behaviors::{BrowserBehavior, DriverBehavior, TabBehavior};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

pub mod behaviors;
pub mod chrome;

pub struct Browser<T: DriverBehavior>
{
    inner: T,
    pub(crate) address: String,
    pub(crate) port: usize,
}
impl<'a, T: DriverBehavior> Deref for Browser<T>
{
    type Target = T;
    fn deref(&self) -> &Self::Target
    {
        &self.inner
    }
}
impl<'a, T: DriverBehavior> DerefMut for Browser<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.inner
    }
}

pub struct Tab<State: DriverBehavior>
{
    state: std::marker::PhantomData<State>,
    ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>,
}