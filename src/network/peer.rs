use std::io::Error;
use std::net::{IpAddr, UdpSocket};
use std::time::Duration;
use log::{error};

use crate::protocol::dns_packet::DnsPacket;
use crate::protocol::packet_buffer;


pub fn nslookup(ip: IpAddr, port: u16, query: &DnsPacket) -> Result<DnsPacket, Error> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", 0));

    if let Err(e) = socket {
        error!("Failed to bind to socket: {}", e);
        return Err(e);
    }

    let socket = socket.unwrap();

    match socket.connect(format!("{}:{}", ip, port)) {
        Err(e) => error!("Failed to connect to {}:{}. Error: {}", ip, port, e),
        Ok(_) => (),
    }


    match socket.set_read_timeout(Some(Duration::from_secs(5))) {
        Err(e) => error!("Failed to set read timeout: {}", e),
        Ok(_) => ()
    }

    let response: DnsPacket;
    loop {
        let input = query.serialize().unwrap();
        match socket.send(&input.buffer) {
            Err(e) => {
                error!("Failed to send query: {}", e);
                continue;
            }
            Ok(_) => (),
        }

        let mut buffer = [0u8; 512];
        match socket.recv_from(&mut buffer){
            Err(e) => {
                error!("Failed to receive response: {}", e);
                continue;
            }
            Ok(_) => ()
        }

        let mut packet_buffer = packet_buffer::PacketBuffer::new(buffer);

        response = DnsPacket::deserialize(&mut packet_buffer).unwrap();


        break;
    }

    drop(socket);
    return Ok(response);
}
