use std::net::Ipv4Addr;

use drivers::behaviors::{DriverReadBehavior, DriverWriteBehavior};

use tools::tab::Tab;

use crate::utils::types::result::Result;

pub mod drivers;
pub mod rw;
pub mod tools;

pub struct Browser<D: DriverReadBehavior + DriverWriteBehavior + Send> {
    driver: std::marker::PhantomData<D>,
    process: tokio::process::Child,
    ipv4addr: Ipv4Addr,
    port: u16,
}
impl<D: DriverReadBehavior + DriverWriteBehavior + Send> Drop for Browser<D> {
    fn drop(&mut self) {
        _ = self.process.start_kill();
    }
}
impl<D: DriverReadBehavior + DriverWriteBehavior + Send> Browser<D> {
    pub async fn open(port: u16) -> Result<(Self, Tab<D>)> {
        //The browser should be open on the local machine
        const LOCAL_ADDRESS: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

        //Build the instance
        Ok((
            Self {
                driver: std::marker::PhantomData::<D>,
                process: D::open_process(LOCAL_ADDRESS, port),
                ipv4addr: LOCAL_ADDRESS,
                port,
            },
            Tab::init(LOCAL_ADDRESS, port).await.unwrap(),
        ))
    }
    pub async fn new_tab(&self) -> Result<Tab<D>> {
        D::new_tab(self.ipv4addr, self.port).await
    }
}

pub enum By {
    Id(String),
    XPath(String),
}
