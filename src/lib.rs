mod utils;
pub use utils::*;
mod logic;
pub use logic::*;

#[cfg(test)]
mod tests
{
    use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, path::Path};

    use server::{
        behavior::{ServerBehavior, ServerDatabaseBehavior}, database::DBValueType, private::Private, public::Public, Server
    };
    use types::result::Result;

    use super::*;

    #[test]
    fn test() -> Result<()>
    {
        let mut server: Server<Private> = Server::default();
        let _ = server.connect(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut volatile = server.create_database("VolatileDB", true);
        volatile.add(&[100, 10], DBValueType::String("".into()));
        let non_volatile = server.create_database("NonVolatileDB", false);


        
        Ok(())
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
