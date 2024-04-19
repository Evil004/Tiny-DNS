use std::{future::Future, io, net::IpAddr};

use crate::{network::udp_server::Server, resolver::ResolverType};

pub trait ServerBuilder {
    fn new() -> Self;
    fn build(&self) -> impl Future<Output = Result<Server, std::io::Error>> + Send;

    fn set_port(&self, port: u16) -> Self;
    fn set_bind_address(&self, bind_address: IpAddr) -> Self;
    fn set_resolver(&self, resolver: ResolverType) -> Self;
}

pub struct ServerBuilderImpl {
    port: Option<u16>,
    bind_address: Option<IpAddr>,
    resolver: Option<ResolverType>,
}

impl ServerBuilder for ServerBuilderImpl {
    fn new() -> Self {
        ServerBuilderImpl {
            port: None,
            bind_address: None,
            resolver: None,
        }
    }

    fn build(&self) -> impl Future<Output = Result<Server, io::Error>> + Send {
        let port = self.port.unwrap_or(53);
        let bind_address = self
            .bind_address
            .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));

        let resolver = self
            .resolver
            .as_ref()
            .unwrap_or(&ResolverType::Cache);

        Server::new(bind_address, port, resolver.clone())
    }

    fn set_port(&self, port: u16) -> Self {
        ServerBuilderImpl {
            port: Some(port),
            ..*self
        }
    }

    fn set_bind_address(&self, bind_address: IpAddr) -> Self {
        ServerBuilderImpl {
            bind_address: Some(bind_address),
            ..*self
        }
    }

    fn set_resolver(&self, resolver: ResolverType) -> Self {
        ServerBuilderImpl {
            resolver: Some(resolver),
            ..*self
        }
    }
}
