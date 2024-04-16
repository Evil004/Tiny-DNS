use crate::parsing::Result;

use super::{dns_record::Class, packet_buffer::PacketBuffer};

#[derive(Debug, Clone)]
pub struct DnsQuery {
    pub domain_names: Vec<String>,
    pub qtype: u16,
    pub qclass: Class,
}

impl DnsQuery {
    pub fn deserialize(packet_buffer: &mut PacketBuffer, query_count: u16) -> Result<Self> {
        let mut domain_names = Vec::new();

        for _ in 0..query_count {
            let domain_name = packet_buffer.read_qname()?;
            domain_names.push(domain_name);
        }

        dbg!(domain_names.clone());

        let qtype = packet_buffer.read_u16()?;
        dbg!(qtype.clone());

        let qclass = Class::deserialize(packet_buffer)?;

        return Ok(DnsQuery {
            domain_names,
            qtype,
            qclass,
        });
    }

    pub fn serialize(&self, packet_buffer: &mut PacketBuffer) -> Result<()> {
        for name in &self.domain_names {
            packet_buffer.write_qname(name)
        }

        packet_buffer.write_u16(self.qtype);
        packet_buffer.write_u16(self.qclass.into());

        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use crate::protocol::dns_record::Class;


    #[test]
    fn serialize_and_deserialize_dns_query() {
        use super::DnsQuery;
        use crate::protocol::packet_buffer::PacketBuffer;

        let mut packet_buffer = PacketBuffer::new([0; 512]);

        let domain_names = vec![
            "goole.com".to_string(),
            "images.google.com".to_string(),
            "www.images.google.com".to_string(),
        ];

        let dns_query = DnsQuery {
            domain_names,
            qtype: 1,
            qclass: Class::IN,
        };

        dns_query.serialize(&mut packet_buffer).unwrap();

        packet_buffer.pos = 0;

        let dns_query = DnsQuery::deserialize(&mut packet_buffer, 3).unwrap();

        assert_eq!(
            dns_query.domain_names,
            vec![
                "goole.com".to_string(),
                "images.google.com".to_string(),
                "www.images.google.com".to_string()
            ]
        );
        assert_eq!(dns_query.qtype, 1);
        assert_eq!(dns_query.qclass, Class::IN);
    }
}
