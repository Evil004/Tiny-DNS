use std::{io, sync::Arc};

use tokio::net::UdpSocket;

use crate::protocol::{dns_packet::DnsPacket, packet_buffer::PacketBuffer};

use super::udp_client::nslookup;


pub struct Server {
    socket: Arc<UdpSocket>,
}

impl Server {
    pub async fn new() -> io::Result<Server> {
        let socket = UdpSocket::bind("0.0.0.0:5300").await?;

        return Ok(Server {
            socket: Arc::new(socket),
        });
    }

    pub async fn start(&self) -> io::Result<()> {
        let mut buf = [0u8; 512];

        loop {
            let socket = self.socket.clone();

            if let Ok((_, _src)) = socket.clone().recv_from(&mut buf).await {
                tokio::task::spawn(async move {
                    let response = handle_query(buf);

                    socket
                        .send_to(&response, &_src)
                        .await
                        .expect("Failed to send response");
                });
            }
        }
    }
}

fn handle_query<'a>(buf: [u8; 512]) -> Vec<u8> {

    let mut packet_buffer = PacketBuffer::new(buf);

    let query = DnsPacket::deserialize(&mut packet_buffer).unwrap();

    let response = nslookup(query);

    let packet_buffer = response.serialize().unwrap();

    return packet_buffer.buffer[..packet_buffer.pos].to_vec();
}
