use std::net::{IpAddr, UdpSocket};
use std::time::Duration;
use crate::errors::LookupError;

use crate::protocol::dns_packet::DnsPacket;
use crate::protocol::packet_buffer;


pub fn nslookup(ip: IpAddr, port: u16, query: &DnsPacket) -> Result<DnsPacket, LookupError> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", 0));

    if let Err(e) = socket {
        return Err(LookupError::FailedToBindSocket(e));
    }

    let socket = socket.unwrap();

    match socket.connect(format!("{}:{}", ip, port)) {
        Err(_) => {
            return Err(LookupError::FailedToConnectSocket {
                ip,
                port,
            });
        }
        Ok(_) => (),
    }


    match socket.set_read_timeout(Some(Duration::from_secs(5))) {
        Err(_) => {
            return Err(LookupError::FailedToSetReadTimeout);
        }
        Ok(_) => ()
    }

    let response: DnsPacket;
    let input = query.serialize().unwrap();
    match socket.send(&input.buffer) {
        Err(_) => {
            return Err(LookupError::FailedToSendQuery);
        }
        Ok(_) => (),
    }

    let mut buffer = [0u8; 512];
    match socket.recv_from(&mut buffer) {
        Err(e) => {
            return Err(LookupError::FailedToReceiveResponse(e));
        }
        Ok(_) => ()
    }

    let mut packet_buffer = packet_buffer::PacketBuffer::new(buffer);

    response = DnsPacket::deserialize(&mut packet_buffer).unwrap();


    drop(socket);
    return Ok(response);
}
