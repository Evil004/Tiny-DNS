use std::{io, net::IpAddr, sync::Arc};

use tokio::net::UdpSocket;

use crate::{
    protocol::{dns_packet::DnsPacket, packet_buffer::PacketBuffer},
    resolver::ResolverType,
};

pub struct Server {
    socket: Arc<UdpSocket>,
    resolver: Arc<ResolverType>,
}

impl Server {
    pub async fn new(
        bind_address: IpAddr,
        port: u16,
        resolver: ResolverType,
    ) -> io::Result<Server> {
        let socket = UdpSocket::bind((bind_address, port)).await?;

        return Ok(Server {
            socket: Arc::new(socket),
            resolver: Arc::new(resolver),
        });
    }

    pub async fn start(&self) -> io::Result<()> {
        let mut buf = [0u8; 512];

        loop {
            let socket = self.socket.clone();

            if let Ok((_, _src)) = socket.clone().recv_from(&mut buf).await {
                let resolver = self.resolver.clone();

                tokio::task::spawn(async move {
                    let response = match handle_query(buf, resolver.as_ref()) {
                        Ok(response) => response,
                        Err(e) => {
                            eprintln!("Failed to handle query: {}", e);
                            return;
                        }
                    };

                    socket
                        .send_to(&response, &_src)
                        .await
                        .expect("Failed to send response");
                });
            }
        }
    }
}

fn handle_query<'a>(buf: [u8; 512], resolver: &ResolverType) -> Result<Vec<u8>, io::Error> {
    let mut packet_buffer = PacketBuffer::new(buf);

    let query = DnsPacket::deserialize(&mut packet_buffer).unwrap();

    let response = resolver.resolve(query)?;
    let packet_buffer = response.serialize().unwrap();

    return Ok(packet_buffer.buffer[..packet_buffer.pos].to_vec());
}
