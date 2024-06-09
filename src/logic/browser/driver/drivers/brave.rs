use crate::browser::Settings;

#[derive(Clone, PartialEq, Default)]
pub struct BraveDriver
{
    settings: Settings,
}
impl BraveDriver
{
    pub fn open(settings: Settings) -> Self
    {
        Self { settings }
    }
}
