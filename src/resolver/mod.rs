use std::net::IpAddr;

use crate::{network::peer::nslookup, protocol::dns_packet::DnsPacket};
use crate::errors::QueryError;


#[derive(Debug, Clone, Copy)]
pub enum ResolverType {
    Mirror { mirror_address: IpAddr, port: u16 }
}
impl ResolverType {
    pub fn resolve(&self, query: DnsPacket) -> Result<DnsPacket, QueryError> {
        match self {
            ResolverType::Mirror { mirror_address, port } => {
                let response = nslookup(*mirror_address, *port, &query);
                
                if let Err(_) = response {
                    return Err(QueryError::FailedToDeserializeResponse);
                }
                let response = response.unwrap();
                
                return Ok(response);
            }
        }
    }
}