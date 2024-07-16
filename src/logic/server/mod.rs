use std::{collections::HashMap, net::IpAddr, sync::Arc};

use behavior::{PrivateServerBehavior, ServerBehavior, ServerDatabaseBehavior};
use database::{
    behavior::DatabaseBehavior,
    memory::{non_volatile::NonVolatile, volatile::Volatile},
};

use crate::types::result::Result;

pub mod behavior;
pub mod database;
pub mod private;
pub mod public;

#[derive(Clone, Debug, Default)]
pub struct Server<T>
where
    T: ServerBehavior,
{
    pub(crate) connection: T,
    pub(crate) databases: HashMap<String, Arc<dyn DatabaseBehavior>>,
}
impl<T: ServerBehavior> ServerBehavior for Server<T>
{
    fn connect(&mut self, ip_addr: IpAddr, port: u16) -> Result<()>
    {
        self.connection.connect(ip_addr, port)
    }
}
impl<T: ServerBehavior> ServerDatabaseBehavior for Server<T>
{
    fn create_database(&mut self, name: &str, volatile: bool) -> Arc<dyn DatabaseBehavior>
    {
        let db: Arc<dyn DatabaseBehavior> = match volatile
        {
            true => Arc::new(Volatile::new(name)),
            false => Arc::new(NonVolatile::new(name)),
        };

        self.databases.insert(name.to_string(), Arc::clone(&db));
        db
    }

    fn get_database(&self, name: &str) -> Option<&Arc<dyn DatabaseBehavior>>
    {
        self.databases.get(name)
    }
}
