use std::net::IpAddr;

use crate::types::result::Result;

use super::behavior::{PrivateServerBehavior, ServerBehavior};

#[derive(Clone, Debug, Default)]
pub struct Public{
    address: String
}
impl ServerBehavior for Public
{
    fn connect(&mut self, ip_address: IpAddr, port: u16) -> Result<()>
    {
        match ip_address
        {
            IpAddr::V4(addr) => self.connect_ipv4(addr, port),
            IpAddr::V6(addr) => self.connect_ipv6(addr, port),
        }
    }
}
impl PrivateServerBehavior for Public
{
    fn connect_ipv4(&mut self, ip: std::net::Ipv4Addr, port: u16) -> Result<()>
    {
        self.address = format!("{ip}:{port}");
        println!("Public connection using ipv4: {}", self.address);
        todo!()
    }
    fn connect_ipv6(&mut self, ip: std::net::Ipv6Addr, port: u16) -> Result<()>
    {
        self.address = format!("{ip}:{port}");
        println!("Public connection using ipv6: {}", self.address);
        todo!()
    }
}
