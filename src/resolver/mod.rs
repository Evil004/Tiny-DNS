use crate::protocol::{Class, DnsRecord};

pub fn resolv(_domain: &str, _qtype: DnsRecord, _qclass: Class) -> QueryResolved {
    QueryResolved {
        direction: &[192, 168, 1, 1],
    }
}

pub struct QueryResolved<'a> {
    pub direction: &'a [u8],
}
