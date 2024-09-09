use crate::{logic::browser::behaviors::TabBehavior, utils::types::result::Result};

pub struct Tab{

}
impl TabBehavior for Tab {
    fn navigate(url: &str) -> Result<()> {
        todo!()
    }
}