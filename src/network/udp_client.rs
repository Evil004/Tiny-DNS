use std::io::Error;
use std::net::UdpSocket;

use crate::protocol::dns_packet::DnsPacket;
use crate::protocol::packet_buffer;

pub fn nslookup(query: &DnsPacket) -> Result<DnsPacket,Error> {

    let random_port = rand::random::<u16>() % 1000 + 4000;

    let socket = UdpSocket::bind(format!("0.0.0.0:{}", random_port)).expect("Could not bind client socket");
    socket
        .connect("8.8.8.8:53")
        .expect("Could not connect to server");

    socket.set_read_timeout(Some(std::time::Duration::from_secs(5))).expect("Failed to set read timeout");

    let response: DnsPacket;
    loop {
        let input = query.serialize().unwrap();
        socket
            .send(&input.buffer)
            .expect("Failed to write to server");

        let mut buffer = [0u8; 512];
        socket
            .recv_from(&mut buffer)
            .expect("Could not read into buffer");

        let mut packet_buffer = packet_buffer::PacketBuffer::new(buffer);

        response = DnsPacket::deserialize(&mut packet_buffer).unwrap();


        break;
    }

    drop(socket);
    return Ok(response);
}
