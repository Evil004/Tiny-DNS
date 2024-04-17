use crate::protocol::{dns_answer::DnsAnswer, dns_packet::DnsPacket, dns_record::Class};

pub fn resolv(query: &DnsPacket) -> DnsAnswer {


    
    DnsAnswer::new(vec![], Class::IN, 60, vec![])
}
