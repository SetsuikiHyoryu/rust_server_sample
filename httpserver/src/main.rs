use server::Server;

mod handler;
mod router;
mod server;

fn main() {
    let server = Server::new("localhost:8000");
    server.run();
}
