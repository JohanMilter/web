use crate::{browser::ChromeUrl, Attribute};

pub struct ChromeElement<'a>
{
    pub(crate) url_ref: &'a ChromeUrl<'a>,
}
//Actions
impl<'a> ChromeElement<'a> {
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
