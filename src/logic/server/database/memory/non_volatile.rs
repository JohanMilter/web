
use crate::server::database::{
    behavior::{DatabaseActionBehavior, DatabaseBehavior},
    DBValueType,
};

#[derive(Clone, Debug, Default)]
pub struct NonVolatile
{
    pub(crate) name: String,
}
impl DatabaseBehavior for NonVolatile
{
    fn new(name: &str) -> Self
    {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    fn get_name(&self) -> &str
    {
        &self.name
    }

    fn get_type(&self) -> &str
    {
        "NonVolatile"
    }
}
impl DatabaseActionBehavior for NonVolatile
{
    fn add(&mut self, indices: &[usize], value: DBValueType)
    {
        todo!("Create the data in files")
    }
}
