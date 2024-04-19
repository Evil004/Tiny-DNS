use std::io::Error;

use crate::{network::udp_client::nslookup, protocol::{dns_header::{DnsHeader, ResponseCode}, dns_packet::DnsPacket}};

pub fn resolv(query: &DnsPacket) -> Result<DnsPacket, Error> {

    if !query.header.recursion_desired{
        return Ok(create_nxdomain_response(query));
    }

    let response = nslookup(query)?;
    return Ok(response);
}

pub fn create_nxdomain_response(query: &DnsPacket) -> DnsPacket {
    let header = DnsHeader {
        is_response: true,
        rcode: ResponseCode::NXDomain,
        arcount: 0,
        ..query.header.clone()
    };

    let response = DnsPacket {
        header,
        questions: query.questions.clone(),
        answers: Vec::new(),
        authority: Vec::new(),
        additional: Vec::new(),
    };

    return response;
}