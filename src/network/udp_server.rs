use std::{net::IpAddr, sync::Arc};
use log::{error, info};

use tokio::net::UdpSocket;

use crate::{
    protocol::{dns_packet::DnsPacket, packet_buffer::PacketBuffer},
    resolver::ResolverType,
};
use crate::errors::{QueryError, ServerError};

pub struct Server {
    socket: Arc<UdpSocket>,
    resolver: Arc<ResolverType>,
}

impl Server {
    pub async fn new(
        bind_address: IpAddr,
        port: u16,
        resolver: ResolverType,
    ) -> Result<Server, ServerError> {
        let socket = UdpSocket::bind((bind_address, port)).await;

        if let Err(_) = socket {
            return Err(ServerError::FailedToBindSocket);
        }
        let socket = socket.unwrap();

        return Ok(Server {
            socket: Arc::new(socket),
            resolver: Arc::new(resolver),
        });
    }

    pub async fn start(&self) -> Result<(), ServerError> {
        let mut buf = [0u8; 512];

        info!("Server started");
        loop {
            let socket = self.socket.clone();

            if let Ok((_, _src)) = socket.clone().recv_from(&mut buf).await {
                let resolver = self.resolver.clone();

                tokio::task::spawn(async move {
                    let response = match handle_query(buf, resolver.as_ref()) {
                        Ok(response) => response,
                        Err(e) => {
                            error!("Failed to handle query: {}", e);
                            return;
                        }
                    };

                    let send_result = socket
                        .send_to(&response, &_src)
                        .await;

                    if let Err(e) = send_result {
                        error!("Failed to send response: {}", e);
                    }
                });
            }
        }
    }
}

fn handle_query<'a>(buf: [u8; 512], resolver: &ResolverType) -> Result<Vec<u8>, QueryError> {
    let mut packet_buffer = PacketBuffer::new(buf);

    let query = DnsPacket::deserialize(&mut packet_buffer);
    if let Err(e) = query {
        error!("{}", e);
        return Err(QueryError::FailedToDeserializeResponse);
    }
    let query = query.unwrap();

    let response = resolver.resolve(query);
    if let Err(e) = response {
        error!("{}", e);
        return Err(QueryError::FailetToResolveQuery);
    }
    let response = response.unwrap();
    
    let packet_buffer = response.serialize();
    
    if let Err(e) = packet_buffer {
        error!("{}", e);
        return Err(QueryError::FailedToSerializeResponse);
    }
    let packet_buffer = packet_buffer.unwrap();

    return Ok(packet_buffer.buffer[..packet_buffer.pos].to_vec());
}
