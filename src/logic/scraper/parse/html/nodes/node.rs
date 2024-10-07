
use super::super::nodes::{child::Child, comment::Comment, declaration::Declaration};
use super::super::properties::content::Content;
use super::super::is;

pub enum Node
{
    Child(Child),
    Comment(Comment),
    Declaration(Declaration),
}
impl Node
{
    pub fn get_identifier(&self) -> &str
    {
        match self
        {
            Node::Child(child) => child.get_inner_html(),
            Node::Declaration(declaration) => declaration.get_inner_html(),
            Node::Comment(comment) => comment.get_inner_html(),
        }
    }
    pub fn get_type(&self) -> &str
    {
        match self
        {
            Node::Child(_) => "child",
            Node::Declaration(_) => "declaration",
            Node::Comment(_) => "comment",
        }
    }
    pub(crate) fn push(&mut self, node: Node)
    {
        match self
        {
            Self::Declaration(_) =>
            {}
            Self::Comment(_) =>
            {}
            Self::Child(child) =>
            {
                child.push(node);
            }
        }
    }
    pub(crate) fn push_content(&mut self, content: String)
    {
        if let Node::Child(child) = self
        {
            child.content = Some(Content::new(content));
        }
    }
    pub(crate) fn from_header(header: String) -> Self
    {
        if is::comment(&header)
        {
            Self::Comment(Comment::from_inner_html(header))
        }
        else if is::declaration(&header)
        {
            Self::Declaration(Declaration::from_inner_html(header))
        }
        else if is::child(&header)
        {
            Self::Child(Child::from_inner_html(header))
        }
        else
        {
            panic!("Invalid node")
        }
    }
}
