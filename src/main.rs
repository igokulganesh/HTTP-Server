mod server;
mod http;

use server::Server;

fn main() {
    let server: Server = Server::new("127.0.0.1:5070".to_string());
    server.run()
}