use behaviors::BrowserBehavior;

use crate::utils::types::result::Result;

pub mod behaviors;
pub mod chrome;
pub struct Browser<T>
where
    T: BrowserBehavior,
{
    pub(crate) inner: Result<T>,
}
impl<T> Browser<T>
where
    T: BrowserBehavior,
{
    pub async fn open() -> Self
    {
        Self {
            inner: T::open(r"C:\Program Files\Google\Chrome\Application\chrome.exe").await,
        }
    }
    pub async fn kill(&mut self)
    {
        self.inner.as_mut().unwrap().kill().await;
    }
}
