use crate::browser::Settings;

#[derive(Clone, PartialEq, Default)]
pub struct EdgeDriver
{
    settings: Settings,
}
impl EdgeDriver
{
    pub fn open(settings: Settings) -> Self
    {
        Self { settings }
    }
}
