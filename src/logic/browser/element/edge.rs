use crate::{browser::EdgeUrl, Attribute};

pub struct EdgeElement<'a>
{
    pub(crate) url_ref: &'a EdgeUrl<'a>,
}
//Actions
impl<'a> EdgeElement<'a> {
    pub fn click(&self){
        println!("Clicking element");
    }
    pub fn write(&self){
        println!("Writing element");
    }
    pub fn read(&self) -> Vec<Attribute>{
        println!("Reading element");
        Vec::new()
    }
}
