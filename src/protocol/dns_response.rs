use super::{
    dns_answer::DnsAnswer, dns_header::DnsHeader, dns_query::DnsQueryPacket,
    dns_question::DnsQuestion,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DnsResponsePacket {
    header: DnsHeader,
    querys: DnsQuestion,
    answers: DnsAnswer,
}

impl DnsResponsePacket {
    pub fn from_query(query: DnsQueryPacket, ttl: u32, rdata: Vec<u8>) -> DnsResponsePacket {

        let answers = DnsAnswer::from_query(&query.question, ttl, rdata);

        return DnsResponsePacket {
            header: query.header,
            querys: query.question,
            answers: answers,
        };
    }
  
}
