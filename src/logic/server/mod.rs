use std::collections::HashMap;

use database::{
    memory::{non_volatile::NonVolatileDB, volatile::VolatileDB},
    Database,
};

pub mod database;
pub mod private;
pub mod public;

#[derive(Clone, Debug, Default)]
pub struct Server<T>
{
    pub(crate) connection: T,
    pub(crate) databases: HashMap<String, Database>,
}
impl<T> Server<T>
{
    pub fn create_database(&mut self, name: &str, volatile: bool) -> &mut Database
    {
        let db = match volatile
        {
            true => Database::Volatile(VolatileDB::new(name)),
            false => Database::NonVolatile(NonVolatileDB::new(name)),
        };
        self.databases.insert(name.to_string(), db);
        self.databases.get_mut(name).unwrap()
    }
}
