extern crate nom;
extern crate peel_ip;
use peel_ip::prelude::*;

static TLS_HEADER: &'static [u8] =
    &[0x16, 0x03, 0x01, 0x00, 0xf4, 0x01, 0x00, 0x00, 0xf0, 0x03, 0x03, 0x14, 0x5b, 0x92, 0xc3, 0xcd, 0x27, 0xe0,
      0xa7, 0x09, 0x1d, 0x3a, 0x14, 0xda, 0x13, 0x8f, 0x19, 0x92, 0x9b, 0x5f, 0xd9, 0x75, 0x34, 0xe7, 0x45, 0xd8,
      0x2d, 0x1c, 0xa9, 0xb0, 0x89, 0x3c, 0xac, 0x20, 0x58, 0x44, 0x00, 0x00, 0x68, 0x46, 0xcb, 0x02, 0xee, 0xfd,
      0x82, 0x22, 0x32, 0x12, 0x89, 0x20, 0x73, 0xbe, 0x5d, 0x4b, 0xdb, 0x0b, 0xe5, 0x2f, 0x2c, 0xf6, 0x41, 0x1f,
      0x27, 0xcb, 0xf1, 0x21, 0x00, 0x20, 0xc0, 0x2b, 0xc0, 0x2f, 0x00, 0x9e, 0xcc, 0x14, 0xcc, 0x13, 0xcc, 0x15,
      0xc0, 0x0a, 0xc0, 0x14, 0x00, 0x39, 0xc0, 0x09, 0xc0, 0x13, 0x00, 0x33, 0x00, 0x9c, 0x00, 0x35, 0x00, 0x2f,
      0x00, 0x0a, 0x01, 0x00, 0x00, 0x87, 0xff, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x16, 0x00, 0x14, 0x00,
      0x00, 0x11, 0x61, 0x73, 0x65, 0x63, 0x75, 0x72, 0x69, 0x74, 0x79, 0x73, 0x69, 0x74, 0x65, 0x2e, 0x63, 0x6f,
      0x6d, 0x00, 0x17, 0x00, 0x00, 0x00, 0x23, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x16, 0x00, 0x14, 0x06, 0x01, 0x06,
      0x03, 0x05, 0x01, 0x05, 0x03, 0x04, 0x01, 0x04, 0x03, 0x03, 0x01, 0x03, 0x03, 0x02, 0x01, 0x02, 0x03, 0x00,
      0x05, 0x00, 0x05, 0x01, 0x00, 0x00, 0x00, 0x00, 0x33, 0x74, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x10,
      0x00, 0x1d, 0x00, 0x1b, 0x08, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x31, 0x2e, 0x31, 0x08, 0x73, 0x70, 0x64, 0x79,
      0x2f, 0x33, 0x2e, 0x31, 0x05, 0x68, 0x32, 0x2d, 0x31, 0x34, 0x02, 0x68, 0x32, 0x75, 0x50, 0x00, 0x00, 0x00,
      0x0b, 0x00, 0x02, 0x01, 0x00, 0x00, 0x0a, 0x00, 0x06, 0x00, 0x04, 0x00, 0x17, 0x00, 0x18];

#[test]
fn parse_tls_success() {
    let mut parser = TlsParser;
    println!("{}", parser);
    let parsing_result = parser.parse(TLS_HEADER, None, None).unwrap().1;
    let res = parsing_result.downcast_ref();
    assert_eq!(Some(&TlsPacket {
                   content_type: TlsRecordContentType::Handshake,
                   version: TlsRecordVersion {
                       major: 3,
                       minor: 1,
                   },
                   length: 244,
               }),
               res);
}

#[test]
fn parse_tls_success_content_types() {
    let mut parser = TlsParser;
    // Handshake
    let mut input = Vec::from(TLS_HEADER);
    parser.parse(&input, None, None).unwrap();

    // Check the other types
    for i in 20..25 {
        input[0] = i;
        parser.parse(&input, None, None).unwrap();
    }
}

#[test]
fn parse_tls_failure_content_type() {
    let mut parser = TlsParser;
    // Handshake
    let mut input = Vec::from(TLS_HEADER);
    input[0] = 0;
    assert!(parser.parse(&input, None, None).to_full_result().is_err());
}

#[test]
fn parse_tls_failure_too_small() {
    let mut parser = TlsParser;
    let input = [20, 0];
    assert!(parser.parse(&input, None, None).to_full_result().is_err());
}
