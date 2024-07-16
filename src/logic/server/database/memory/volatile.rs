use crate::{server::database::DatabaseValueType, types::tensor::Tensor};

#[derive(Clone, Debug)]
pub struct VolatileDB
{
    pub(crate) name: String,
    pub(crate) data: Tensor<DatabaseValueType>,
}
impl VolatileDB
{
    pub fn new(name: &str) -> Self
    {
        Self {
            name: name.to_string(),
            data: Tensor::default(),
        }
    }
}