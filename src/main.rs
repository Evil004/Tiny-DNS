use network::udp_server::Server;

mod network;
mod parsing;
mod protocol;
mod resolver;

#[tokio::main]
async fn main() {
    let server = Server::new().await.expect("Failed to create server");

    server.start().await.expect("Failed to start server");
}
