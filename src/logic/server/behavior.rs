use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    sync::Arc,
};

use crate::types::result::Result;

use super::database::behavior::DatabaseBehavior;

pub trait ServerBehavior
{
    fn connect(&mut self, ip_addr: IpAddr, port: u16) -> Result<()>;
}
pub(crate) trait PrivateServerBehavior
{
    fn connect_ipv4(&mut self, ip: Ipv4Addr, port: u16) -> Result<()>;
    fn connect_ipv6(&mut self, ip: Ipv6Addr, port: u16) -> Result<()>;
}
pub trait ServerDatabaseBehavior
{
    fn create_database(&mut self, name: &str, volatile: bool) -> Arc<dyn DatabaseBehavior>;
    fn get_database(&self, name: &str) -> Option<&Arc<dyn DatabaseBehavior>>;
}
