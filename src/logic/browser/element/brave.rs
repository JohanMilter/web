use crate::{browser::BraveUrl, Attribute};

pub struct BraveElement<'a>
{
    pub(crate) url_ref: &'a BraveUrl<'a>,
}
//Actions
impl<'a> BraveElement<'a> {
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
