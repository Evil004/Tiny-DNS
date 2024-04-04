pub fn resolv(_domain: &str, _qtype: u16, _qclass: u16)-> QueryResolved {

    QueryResolved {
        direction: &[192, 168, 1, 1]
    }
}

pub struct QueryResolved<'a> {
    pub direction: &'a [u8],
}