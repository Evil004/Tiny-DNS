use network::udp_server::Server;

mod protocol;
mod network;
mod parsing;

#[tokio::main]
async fn main() {
    let server = Server::new().await.expect("Failed to create server");
    server.start().await.expect("Failed to start server");
}


