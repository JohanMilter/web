use std::fmt::Display;

use crate::*;

pub struct Content(String);
impl Content
{
    pub fn new(content: String) -> Self
    {
        Self(content)
    }
    pub fn as_str(&self) -> &str
    {
        &self.0
    }
}
impl Display for Content
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.0)
    }
}
