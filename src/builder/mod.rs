use std::{future::Future, net::IpAddr};
use std::net::Ipv4Addr;
use log::error;

use crate::{network::udp_server::Server, resolver::ResolverType};
use crate::errors::ConfigError;

pub trait ServerBuilder {
    fn new() -> Self;
    fn build(&self) -> impl Future<Output=Result<Server, ConfigError>> + Send;

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

    async fn build(&self) -> Result<Server, ConfigError> {
        let port = self.port.unwrap_or(53);
        let bind_address = self
            .bind_address
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        let resolver = self
            .resolver
            .as_ref();

        if let None = resolver {
            error!("{}", ConfigError::NoResolverEspecified);
            return Err(ConfigError::NoResolverEspecified);
        }

        match Server::new(bind_address, port, resolver.unwrap().clone()).await {
            Ok(server) => Ok(server),
            Err(_) => {
                Err(ConfigError::FailedToCreateServer)
            }
        }
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