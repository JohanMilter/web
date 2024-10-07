pub struct Comment
{
    pub(crate) inner_html: String,
}
impl Comment
{
    pub fn get_inner_html(&self) -> &str
    {
        &self.inner_html
    }
    pub fn get_raw(&self) -> &str
    {
        &self.inner_html[4..self.inner_html.len() - 3]
    }
    pub(crate) fn from_inner_html(inner_html: String) -> Self
    {
        Self { inner_html }
    }
}
