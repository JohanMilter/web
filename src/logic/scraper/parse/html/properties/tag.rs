use super::super::contains;


pub struct Tag<'a>(&'a str);
impl<'a> Tag<'a>
{
    pub fn as_str(&self) -> &str
    {
        self.0
    }
    pub fn new(header: &'a str) -> Self
    {
        if contains::attributes(header)
        {
            Self(header.split_once(' ').unwrap().0)
        }
        else
        {
            Self(header)
        }
    }
}
