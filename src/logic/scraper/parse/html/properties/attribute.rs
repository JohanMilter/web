use std::fmt::Display;

use crate::*;

pub struct Attribute<'a>
{
    pub key: &'a str,
    pub val: &'a str,
}
impl<'a> Attribute<'a> {}
impl<'a> Display for Attribute<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.key, self.val)
    }
}