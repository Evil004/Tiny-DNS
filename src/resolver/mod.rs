use crate::protocol::{Class, Type};

pub fn resolv(_domain: &str, _qtype: Type, _qclass: Class) -> QueryResolved {
    QueryResolved {
        direction: &[192, 168, 1, 1],
    }
}

pub struct QueryResolved<'a> {
    pub direction: &'a [u8],
}
