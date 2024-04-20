use std::net::{IpAddr, Ipv4Addr};

use tiny_dns::{
    builder::{ServerBuilder, ServerBuilderImpl},
    resolver::ResolverType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .await?;

    server.start().await.expect("Failed to start server");

    Ok(())
}
