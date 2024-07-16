mod utils;
pub use utils::*;
mod logic;
pub use logic::*;

#[cfg(test)]
mod tests
{
    use std::path::Path;

    use server::{private::Private, Server};
    use types::result::Result;

    use super::*;

    #[test]
    fn test() -> Result<()>
    {
        let mut server: Server<Private> = Server::default();
        let db = server.create_database("SomeDatabase", true);
        
        Ok(())
    }

    #[test]
    fn generate_dir_tree() {
        let connector = chart::file_tree::Connectors::default();
        chart::file_tree::create_dir_tree_file(&connector, &Some(chart::ignore!["target", ".git"]), Path::new(r"P:\Languages\Rust\Libs\web"), Path::new(r"P:\Languages\Rust\Libs\web\docs"))
    }
}
