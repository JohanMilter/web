use crate::{server::database::DatabaseValueType, types::tensor::Tensor};

#[derive(Clone, Debug)]
pub struct NonVolatileDB
{
    pub(crate) name: String,
    pub(crate) data: Tensor<DatabaseValueType>,
}
impl NonVolatileDB
{
    pub fn new(name: &str) -> Self
    {
        Self {
            name: name.to_string(),
            data: Tensor::default(),
        }
    }
}
