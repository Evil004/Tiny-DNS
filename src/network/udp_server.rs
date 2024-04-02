use std::{io, sync::Arc};

use tokio::net::UdpSocket;

use crate::protocol::{dns_query::DnsQueryPacket, dns_response::DnsResponsePacket};

pub struct Server {
    socket: Arc<UdpSocket>,
}

impl Server {
    pub async fn new() -> io::Result<Server> {
        let socket = UdpSocket::bind("0.0.0.0:53").await?;
        return Ok(Server {
            socket: Arc::new(socket),
        });
    }

    pub async fn start(&self) -> io::Result<()> {
        let mut buf = [0u8; 1500];

        loop {
            let socket = self.socket.clone();

            if let Ok((num_bytes, _src)) = socket.clone().recv_from(&mut buf).await {
                tokio::task::spawn(async move {
                    handle_query(buf[..num_bytes].as_ref());
                });
            }
        }
    }
}

fn handle_query(buf: &[u8]) {
    let (_, query) = DnsQueryPacket::parse_query_from_bit_input((buf, 0)).unwrap();

    let response = DnsResponsePacket::from_query(query, 600, vec![192, 168, 1, 1]);

    println!("{:?}", response);
}
