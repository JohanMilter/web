use super::super::properties::{attributes::Attributes, tag::Tag};

pub struct Declaration {
    pub(crate) inner_html: String,
}
impl Declaration {
    pub fn get_inner_html(&self) -> &str {
        &self.inner_html
    }
    pub(crate) fn from_inner_html(inner_html: String) -> Self {
        Self { inner_html }
    }
    pub fn get_tag(&self) -> Option<Tag> {
        Some(Tag::new(&self.inner_html[2..]))
    }
    pub fn get_attributes(&self) -> Option<Attributes> {
        Attributes::new(&self.inner_html)
    }
}
