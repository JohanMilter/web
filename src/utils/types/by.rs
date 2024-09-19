pub enum By<'a> {
    Id(&'a str),
    XPath(&'a str),
}
