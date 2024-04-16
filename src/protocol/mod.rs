pub mod dns_answer;
pub mod dns_header;
pub mod dns_packet;
pub mod dns_query;
pub mod packet_buffer;
pub mod dns_record;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
