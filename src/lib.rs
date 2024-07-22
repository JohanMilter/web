pub mod logic;
pub mod utils;

#[cfg(test)]
mod tests
{
    use std::path::Path;

    use super::*;

    #[test]
    fn test()
    {
        
    }

    #[test]
    fn generate_dir_tree()
    {
        let connector = chart::file_tree::Connectors::default();
        chart::file_tree::create_dir_tree_file(
            &connector,
            &Some(chart::ignore!["target", ".git"]),
            Path::new(r"P:\Languages\Rust\Libs\web"),
            Path::new(r"P:\Languages\Rust\Libs\web\docs"),
        )
    }
}
