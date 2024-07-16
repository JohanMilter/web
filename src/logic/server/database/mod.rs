use memory::{non_volatile::NonVolatile, volatile::Volatile};

pub mod behavior;
pub mod memory;

#[derive(Clone, Debug)]
pub enum DBValueType
{
    String(String),
    Int(i32),
    Float(f32),
}
impl Default for DBValueType
{
    fn default() -> Self
    {
        Self::String(String::new())
    }
}

