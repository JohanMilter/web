#[derive(Clone, PartialEq, Debug)]
pub enum UrlFrom<'a>
{
    Url(&'a str),
    Ip(&'a str),
}
