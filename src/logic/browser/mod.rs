use std::ops::{Deref, DerefMut};

use behaviors::BrowserBehavior;
use tab::Tab;

use crate::utils::types::result::Result;

pub mod behaviors;
pub mod chrome;
pub mod tab;

pub struct Browser<'a, T>
where
    T: BrowserBehavior,
{
    pub(crate) inner: Result<T>,
}
impl<'a, T> Browser<'a, T>
where
    T: BrowserBehavior,
{
    pub async fn open() -> Self
    {
        let mut inner = T::open(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
            .await
            .unwrap();


        todo!()
    }
}
impl<'a, T> Deref for Browser<'a, T>
where
    T: BrowserBehavior,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}
impl<'a, T> DerefMut for Browser<'a, T>
where
    T: BrowserBehavior, {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}