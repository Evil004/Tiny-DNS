use std::io::Result;
use std::sync::Arc;
use std::time::Duration;

use nom::bits::complete::take;
use nom::error::Error;
use nom::{bits, IResult};

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let _ = start_server().await;
}

async fn start_server() -> Result<()> {
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:53").await?);

    let mut buf = [0u8; 1500];

    loop {
        let socket = socket.clone();

        if let Ok((num_bytes, _src)) = socket.clone().recv_from(&mut buf).await {
            tokio::task::spawn(async move {
                handle_query(buf[..num_bytes].as_ref());
            });
        }
    }
}

fn handle_query(buf: &[u8]) -> Vec<u8> {
    let (_, query) = dns_query_parser((&buf, 0)).unwrap();
    println!("{:?}", query);
    return vec![];
}

#[derive(Debug)]
struct DnsHeader {
    id: u16,           // 16 bits
    is_response: bool, // 1  bit
    opcode: u8,        // 4  bits

    authoritative_answer: bool, // 1  bit
    truncated_message: bool,    // 1  bit
    recursion_desired: bool,    // 1  bit
    recursion_available: bool,  // 1  bit

    z: u8,           // 4  bit
    response_code: u8, // 4  bits

    question_count: u16, // 16 bits
    answer_count: u16,   // 16 bits
    nscount: u16,        // 16 bits
    arcount: u16,        // 16 bits
}

#[derive(Debug)]
struct DnsQuery {
    header: DnsHeader,
}

type BitInput<'a> = (&'a [u8], usize);

fn dns_query_parser(input: BitInput) -> IResult<BitInput, DnsQuery> {
    let (input, header) = parse_dns_header(input)?;

    let query = DnsQuery { header: header };

    return Ok((input, query));
}

fn parse_dns_header(input: BitInput) -> IResult<BitInput, DnsHeader> {
    let (input, id) = take_16bits(input)?;
    let (input, is_response) = take_1bit_bool(input)?;

    let (input, opcode) = take_4bits(input)?;

    let (input, authoritative_answer) = take_1bit_bool(input)?;
    let (input, truncated_message) = take_1bit_bool(input)?;

    let (input, recursion_desired) = take_1bit_bool(input)?;
    let (input, recursion_available) = take_1bit_bool(input)?;

    let (input, z) = take_3bits(input)?;
    let (input, response_code) = take_4bits(input)?;
    let (input, question_count) = take_16bits(input)?;
    let (input, answer_count) = take_16bits(input)?;
    let (input, nscount) = take_16bits(input)?;
    let (input, arcount) = take_16bits(input)?;

    let header = DnsHeader {
        id,
        is_response,
        opcode,
        authoritative_answer,
        truncated_message,
        recursion_desired,
        recursion_available,
        z,
        response_code,
        question_count,
        answer_count,
        nscount,
        arcount,
    };

    return Ok((input, header));
}

fn take_16bits(input: BitInput) -> IResult<BitInput, u16> {
    let (input, num) = take(16u8)(input)?;
    return Ok((input, num));
}

fn take_4bits(input: BitInput) -> IResult<BitInput, u8> {
    let (input, num) = take(4u8)(input)?;
    return Ok((input, num));
}
fn take_3bits(input: BitInput) -> IResult<BitInput, u8> {
    let (input, num) = take(3u8)(input)?;
    return Ok((input, num));
}
fn take_1bit_bool(input: BitInput) -> IResult<BitInput, bool> {
    let (input, num): (BitInput, u8) = take(1usize)(input)?;

    return Ok((input, num == 1));
}
