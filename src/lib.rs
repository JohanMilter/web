mod components;
macros::import!(logic > pub [*]);

#[cfg(test)]
mod tests
{
    use std::net::TcpListener;

    use super::*;

    #[test]
    fn test()
    {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address!");
        println!("Server listening on 127.0.0.1:8080");

        for stream in listener.incoming(){
            match stream {
                Ok(stream) => {
                    std::thread::spawn(|| server::client::handle(stream));
                }
                Err(e) => {
                    println!("Failed to establish connection: {e}");
                }
            }
        }
    }

    #[test]
    fn generate_dir_tree() {}
}
