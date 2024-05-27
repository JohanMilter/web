


pub trait GetTabs<T>
{
    fn new_tab(&mut self, name: &str) -> T;
    fn pop_tab(&mut self) -> Option<T>;
    fn delete_tab(&mut self, name: &str) -> Option<T>;
}