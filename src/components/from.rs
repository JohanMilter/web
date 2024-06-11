


#[derive(Clone, PartialEq, Debug)]
pub enum ElementFrom<'a>
{
    Id(&'a str),
    XPath(&'a str),
    Class(&'a str),
    Name(&'a str),
}
