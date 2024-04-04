use std::{io, sync::Arc};

use tokio::net::UdpSocket;

use crate::{
    parsing::{deserialize::Deserialize, serialize::Serialize},
    protocol::{dns_query::DnsQueryPacket, dns_response::DnsResponsePacket},
};

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
                    let response = handle_query(buf[..num_bytes].as_ref());

                    socket
                        .send_to(&response, &_src)
                        .await
                        .expect("Failed to send response");
                });
            }
        }
    }
}

fn handle_query(buf: &[u8]) -> Vec<u8> {
    let (_, query) = DnsQueryPacket::deserialize((buf, 0)).unwrap();

    dbg!(&query);

    let response = DnsResponsePacket::from_query(query, 600, vec![192, 168, 1, 1]);

    dbg!(&response);

    let bytes: Vec<u8> = response.serialize().into_vec();


    return bytes;
}
