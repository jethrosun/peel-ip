extern crate nom;
extern crate peel_ip;
use peel_ip::prelude::*;

static UDP_HEADER: &'static [u8] = &[0x00, 0x35, 0xc7, 0xde, 0x00, 0x92, 0xad, 0x1b];

#[test]
fn udp_parser_variant() {
    let parser = UdpParser;
    println!("{:?}", parser.variant());
}

#[test]
fn parse_udp_success() {
    let parser = UdpParser;
    let res = parser.parse(UDP_HEADER, None, None, None).unwrap().1;
    println!("{}", res.0);
    assert_eq!(Layer::Udp(UdpPacket {
                   source_port: 53,
                   dest_port: 51166,
                   length: 146,
                   checksum: 44315,
               }),
               res.0);
}

#[test]
fn parse_udp_failure_too_small() {
    let parser = UdpParser;
    let mut input = Vec::from(UDP_HEADER);
    input.pop();
    let res = parser.parse(&input, None, None, None);
    assert_eq!(res, IResult::Incomplete(Needed::Size(8)));
}
