use crate::{
    server::database::{
        behavior::{DatabaseActionBehavior, DatabaseBehavior},
        DBValueType,
    },
    types::tensor::Tensor,
};

#[derive(Clone, Debug, Default)]
pub struct Volatile
{
    pub(crate) name: String,
    pub(crate) data: Tensor<DBValueType>,
}
impl DatabaseBehavior for Volatile
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
        "Volatile"
    }
}
impl DatabaseActionBehavior for Volatile
{
    fn add(&mut self, indices: &[usize], value: DBValueType)
    {
        self.data.insert_value(indices, value);
    }
}
