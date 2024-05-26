

pub trait Page: Drop{
    fn kill(self);
}