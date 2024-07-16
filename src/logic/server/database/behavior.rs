use std::fmt::Debug;

use super::DBValueType;

pub trait DatabaseBehavior: Debug + Send + Sync + DatabaseActionBehavior
{
    fn new(name: &str) -> Self
    where
        Self: Sized;
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &str;
}
pub trait DatabaseActionBehavior
{
    fn add(&mut self, indices: &[usize], value: DBValueType);
}
