use super::dns_question::DnsQuestion;


#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsAnswer {
    response_type: u16,    // 16 bits
    response_class: u16,   // 16 bits
    ttl: u32,      // 32 bits
    rdlength: u16, // 16 bits
    rdata: Vec<u8>,
}

impl DnsAnswer {

    pub fn from_query(query: &DnsQuestion, ttl: u32, rdata: Vec<u8>) -> DnsAnswer {
        return DnsAnswer {
            response_type: query.qtype,
            response_class: query.qclass,
            ttl: ttl,
            rdlength: rdata.len() as u16,
            rdata: rdata,
        };
    }
}
