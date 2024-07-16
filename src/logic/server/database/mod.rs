use memory::{non_volatile::NonVolatileDB, volatile::VolatileDB};

pub mod memory;

#[derive(Clone, Debug)]
pub enum DatabaseValueType
{
    String(String),
    Int(i32),
    Float(f32),
}
impl Default for DatabaseValueType
{
    fn default() -> Self
    {
        Self::String(String::new())
    }
}

#[derive(Clone, Debug)]
pub enum Database
{
    Volatile(VolatileDB),
    NonVolatile(NonVolatileDB),
}
