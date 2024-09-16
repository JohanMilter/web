use crate::utils::types::result::Result;

use super::Tab;

pub trait DriverBehavior{
    fn open(port: usize, address: &str) -> impl std::future::Future<Output = Result<(Self, Tab<Self>)>> where Self: Sized;
    fn new_target(&mut self) -> impl std::future::Future<Output = Result<Self>> where Self: Sized;
}
pub trait BrowserBehavior<'a, T: DriverBehavior>{
    fn open(port: usize) -> impl std::future::Future<Output = Result<(Self, Tab<T>)>> where Self: Sized;

    //fn get_tab_mut(&mut self) -> impl std::future::Future<Output = Result<Tab<T>>> where Self: Sized;
    //fn get_tab_ref(&self) -> impl std::future::Future<Output = Result<Tab<T>>> where Self: Sized;
    fn new_tab(&mut self) -> impl std::future::Future<Output = Result<Tab<T>>> where Self: Sized;
}
pub trait TabBehavior {
    fn navigate(&mut self, url: &str) -> impl std::future::Future<Output = Result<()>>;
    fn kill(&mut self) -> impl std::future::Future<Output = Result<()>>;
}