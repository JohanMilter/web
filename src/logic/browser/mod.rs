pub enum DriverType<'a>{
    Chrome(&'a str),
    Brave(&'a str),
    Edge(&'a str),
}



pub trait Driver{

}

struct Chrome;
impl Driver for Chrome {
    
}
struct Brave;
impl Driver for Brave {
    
}
struct Edge;
impl Driver for Edge {
    
}

pub fn open(driver: DriverType) -> Box<dyn Driver>{
    todo!()
}