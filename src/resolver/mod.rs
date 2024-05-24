use std::net::IpAddr;

use crate::{network::peer::nslookup, protocol::dns_packet::DnsPacket};


#[derive(Debug, Clone, Copy)]
pub enum ResolverType {
    Mirror { mirror_address: IpAddr, port: u16 }
}
impl ResolverType {
    pub fn resolve(&self, query: DnsPacket) -> Result<DnsPacket, std::io::Error> {
        match self {
            ResolverType::Mirror { mirror_address, port } => {
                let response = nslookup(*mirror_address, *port, &query)?;

                return Ok(response);
            }
        }
    }
}