
pub trait PageNavigation<T>
{
    fn navigate(&mut self, url: &str) -> Option<T>;
}
