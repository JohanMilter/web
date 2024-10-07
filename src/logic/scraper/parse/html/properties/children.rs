use std::{collections::HashMap, ops::Deref};

use super::super::nodes::node::Node;


#[derive(Default)]
pub struct Children(HashMap<String, Vec<Node>>);
impl Children
{
    pub fn get_child(&self, tag: &str, index: usize) -> Option<&Node>
    {
        if let Some(children) = self.0.get(tag)
        {
            return children.get(index);
        }
        None
    }
    pub fn get_child_vec(&self, tag: &str) -> Option<&Vec<Node>>
    {
        self.0.get(tag)
    }
    pub(crate) fn push(&mut self, node: Node)
    {
        let key = match node
        {
            Node::Comment(_) => "comment".to_owned(),
            Node::Declaration(_) => "declaration".to_owned(),
            Node::Child(ref child) => child.get_tag().as_str().to_owned(),
        };
        self.0.entry(key).or_default().push(node);
    }
}
impl Deref for Children
{
    type Target = HashMap<String, Vec<Node>>;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}
