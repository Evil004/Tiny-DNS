use std::net::{IpAddr, Ipv4Addr};
use log::error;

use tiny_dns::{
    builder::{ServerBuilder, ServerBuilderImpl},
    resolver::ResolverType,
};

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Warn).unwrap();

    let server_builder: ServerBuilderImpl = ServerBuilderImpl::new();

    let bind_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let server = server_builder
        .set_port(5300)
        .set_bind_address(bind_address)
        .set_resolver(ResolverType::Mirror {
            mirror_address: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            port: 53,
        })
        .build()
        .await;

    if let Err(e) = server {
        error!("Failed to build server: {}", e);
        return;
    }

    server.unwrap().start().await.unwrap_or_else(|e| {
        error!("Failed to start server: {}", e);
    });

}