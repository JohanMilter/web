use std::ops::{Deref, Index};

use super::super::contains;
use super::super::properties::attribute::Attribute;

use crate::*;

pub struct Attributes<'a>(Vec<Attribute<'a>>);
impl<'a> Attributes<'a>
{
    pub(crate) fn new(header: &'a str) -> Option<Self>
    {
        if !contains::attributes(header)
        {
            return None;
        }
        let mut vec: Vec<Attribute<'a>> = header
        .split_once(' ')
        .unwrap()
        .1
        .split("\" ")
        .map(|attribute| {
            let (key, val) = attribute.split_once('=').unwrap();
            
            Attribute {
                key,
                val: &val[1..],
            }
        })
        .collect();

        //Change last value, since it ends with '"'
        let val = &mut vec.last_mut().unwrap().val;
        *val = &val[..val.len() - 1];

        Some(Self(vec))
    }
    pub fn get_attribute(&self, key: &str) -> Option<&Attribute<'a>>
    {
        self.0.iter().find(|&attribute| attribute.key == key)
    }
}
impl<'a> Deref for Attributes<'a>
{
    type Target = Vec<Attribute<'a>>;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}
impl<'a> Index<usize> for Attributes<'a> {
    type Output = Attribute<'a>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
