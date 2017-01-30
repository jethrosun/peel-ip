extern crate log;
extern crate peel_ip;
use peel_ip::prelude::*;

extern crate time;
use time::Duration;

static PACKET_ETH_IPV4_TCP: &'static [u8] =
    &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x45, 0x00, 0x00, 0x34,
      0x73, 0x22, 0x40, 0x00, 0x3f, 0x06, 0x3a, 0x09, 0x0a, 0x00, 0x00, 0x65, 0x42, 0xc4, 0x41, 0x70, 0xca, 0x45,
      0x01, 0xbb, 0x98, 0x66, 0x5f, 0x0a, 0x44, 0x9d, 0x7f, 0x05, 0x80, 0x10, 0x20, 0x00, 0xbf, 0xf2, 0x00, 0x00,
      0x01, 0x01, 0x08, 0x0a, 0x00, 0x02, 0x2c, 0x2c, 0x63, 0x93, 0xf1, 0x5b];

static PACKET_ETH_IPV6_UDP: &'static [u8] =
    &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x86, 0xdd, 0x60, 0x00, 0x00, 0x00,
      0x00, 0x24, 0x11, 0x40, 0x3f, 0xfe, 0x05, 0x07, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x86, 0xff, 0xfe, 0x05,
      0x80, 0xda, 0x3f, 0xfe, 0x05, 0x01, 0x48, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42,
      0x09, 0x5c, 0x00, 0x35, 0x00, 0x24, 0xf0, 0x09];

static PACKET_ETH_IPV6_ICMP: &'static [u8] =
    &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x86, 0xdd, 0x60, 0x00, 0x00, 0x00,
      0x00, 0x24, 0x3a, 0x40, 0x3f, 0xfe, 0x05, 0x07, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x86, 0xff, 0xfe, 0x05,
      0x80, 0xda, 0x3f, 0xfe, 0x05, 0x01, 0x48, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42,
      0x80, 0x00, 0x41, 0x5c, 0x02, 0x00, 0x0a, 0x00, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6a,
      0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x61, 0x62, 0x63, 0x64, 0x65,
      0x66, 0x67, 0x68, 0x69];

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

static NTP_HEADER: &'static [u8] =
    &[0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0xcc, 0x25, 0xcc, 0x13, 0x2b, 0x02, 0x10, 0x00, 0x00, 0x00, 0x00, 0x01, 0x52, 0x80,
      0x0c, 0x2b, 0x59, 0x00, 0x64, 0x66, 0x84, 0xf4, 0x4c, 0xa4, 0xee, 0xce, 0x12, 0xb8];

static PACKET_ETH_IPV4_IPV6: &'static [u8] =
    &[0xc2, 0x01, 0x42, 0x02, 0x00, 0x00, 0xc2, 0x00, 0x42, 0x02, 0x00, 0x00, 0x08, 0x00, 0x45, 0x00, 0x00, 0x78,
      0x00, 0x09, 0x00, 0x00, 0xff, 0x29, 0xa7, 0x51, 0x0a, 0x00, 0x00, 0x01, 0x0a, 0x00, 0x00, 0x02, 0x60, 0x00,
      0x00, 0x00, 0x00, 0x3c, 0x11, 0x40, 0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x01, 0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x02];

static PACKET_ETH_IPV6_IPV6: &'static [u8] =
    &[0x01, 0x00, 0x5e, 0x00, 0x00, 0x02, 0x00, 0x00, 0x0c, 0x07, 0xac, 0x12, 0x86, 0xdd, 0x60, 0x00, 0x00, 0x00,
      0x00, 0x38, 0x29, 0x00, 0x02, 0x22, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x02, 0x03, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
      0x60, 0x00, 0x00, 0x00, 0x00, 0x10, 0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0xac, 0x10, 0xc7, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0xe0, 0x00, 0x00, 0x02];

static PACKET_ETH_IPV4_IPV4: &'static [u8] =
    &[0xc2, 0x01, 0x57, 0x75, 0x00, 0x00, 0xc2, 0x00, 0x57, 0x75, 0x00, 0x00, 0x08, 0x00, 0x45, 0x00, 0x00, 0x78,
      0x00, 0x14, 0x00, 0x00, 0xff, 0x04, 0xa7, 0x6b, 0x0a, 0x00, 0x00, 0x01, 0x0a, 0x00, 0x00, 0x02, 0x45, 0x00,
      0x01, 0xa5, 0xd6, 0x63, 0x40, 0x00, 0x3f, 0x06, 0x9b, 0xfc, 0xc0, 0xa8, 0x01, 0x0a, 0xad, 0xfc, 0x58, 0x44];

#[test]
fn peel_success_dot() {
    let mut peel = PeelIp::default();
    assert!(peel.create_dot_file().is_ok());
}

#[test]
fn peel_success_tcp() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(PACKET_ETH_IPV4_TCP, vec![]).result;
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].downcast_ref(),
               Some(&EthernetPacket {
                   dst: Default::default(),
                   src: Default::default(),
                   ethertype: EtherType::Ipv4,
               }));
    assert_eq!(result[1].downcast_ref(),
               Some(&Ipv4Packet {
                   version: 4,
                   ihl: 20,
                   tos: 0,
                   length: 52,
                   id: 29474,
                   flags: 2,
                   fragment_offset: 0,
                   ttl: 63,
                   protocol: IpProtocol::Tcp,
                   checksum: 14857,
                   src: Ipv4Addr::new(10, 0, 0, 101),
                   dst: Ipv4Addr::new(66, 196, 65, 112),
               }));
    assert_eq!(result[2].downcast_ref(),
               Some(&TcpPacket {
                   header: TcpHeader {
                       source_port: 51781,
                       dest_port: 443,
                       sequence_no: 2556845834,
                       ack_no: 1151172357,
                       data_offset: 32,
                       reserved: 0,
                       flag_urg: false,
                       flag_ack: true,
                       flag_psh: false,
                       flag_rst: false,
                       flag_syn: false,
                       flag_fin: false,
                       window: 8192,
                       checksum: 49138,
                       urgent_pointer: 0,
                       options: vec![1, 1, 8, 10, 0, 2, 44, 44, 99, 147, 241, 91],
                   },
                   path_error: None,
               }));
}

#[test]
fn peel_success_tls_http() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let mut packet = Vec::from(PACKET_ETH_IPV4_TCP);
    packet.extend_from_slice(TLS_HEADER);
    let result = peel.traverse(&packet, vec![]).result;
    assert_eq!(result.len(), 5);
    assert_eq!(result[3].downcast_ref(),
               Some(&TlsPacket {
                   content_type: TlsRecordContentType::Handshake,
                   version: TlsRecordVersion {
                       major: 3,
                       minor: 1,
                   },
                   length: 244,
               }));
    assert_eq!(result[4].downcast_ref(), Some(&HttpPacket::Any));
}

#[test]
fn peel_success_udp() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(PACKET_ETH_IPV6_UDP, vec![]).result;
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].downcast_ref(),
               Some(&EthernetPacket {
                   dst: Default::default(),
                   src: Default::default(),
                   ethertype: EtherType::Ipv6,
               }));
    assert_eq!(result[1].downcast_ref(),
               Some(&Ipv6Packet {
                   version: 6,
                   traffic_class: 0,
                   flow_label: 0,
                   payload_length: 36,
                   next_header: IpProtocol::Udp,
                   hop_limit: 64,
                   src: Ipv6Addr::new(0x3ffe, 0x507, 0, 1, 0x200, 0x86ff, 0xfe05, 0x80da),
                   dst: Ipv6Addr::new(0x3ffe, 0x501, 0x4819, 0, 0, 0, 0, 0x42),
               }));
    assert_eq!(result[2].downcast_ref(),
               Some(&UdpPacket {
                   header: UdpHeader {
                       source_port: 2396,
                       dest_port: 53,
                       length: 36,
                       checksum: 61449,
                   },
                   path_error: None,
               }));
}

#[test]
fn peel_success_ntp() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let mut packet = Vec::from(PACKET_ETH_IPV6_UDP);
    packet.extend_from_slice(NTP_HEADER);
    let result = peel.traverse(&packet, vec![]).result;
    assert_eq!(result.len(), 4);
    assert_eq!(result[3].downcast_ref(),
               Some(&NtpPacket {
                   li: 0,
                   version: 4,
                   mode: 3,
                   stratum: 0,
                   poll: 0,
                   precision: 0,
                   root_delay: 12,
                   root_dispersion: 0,
                   ref_id: 0,
                   ts_ref: 0,
                   ts_orig: 0,
                   ts_recv: 0,
                   ts_xmit: 14710388140573593600,
                   auth: Some((1, NTP_HEADER[52..].to_vec())),
               }));
}

#[test]
fn peel_success_icmpv6() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(PACKET_ETH_IPV6_ICMP, vec![]).result;
    assert_eq!(result.len(), 3);
}

#[test]
fn peel_success_ipv6_in_ipv4() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(PACKET_ETH_IPV4_IPV6, vec![]).result;
    assert_eq!(result.len(), 3);
    assert_eq!(result[1].downcast_ref(),
               Some(&Ipv4Packet {
                   version: 4,
                   ihl: 20,
                   tos: 0,
                   length: 120,
                   id: 9,
                   flags: 0,
                   fragment_offset: 0,
                   ttl: 255,
                   protocol: IpProtocol::Ipv6,
                   checksum: 42833,
                   src: Ipv4Addr::new(10, 0, 0, 1),
                   dst: Ipv4Addr::new(10, 0, 0, 2),
               }));
    assert_eq!(result[2].downcast_ref(),
               Some(&Ipv6Packet {
                   version: 6,
                   traffic_class: 0,
                   flow_label: 0,
                   payload_length: 60,
                   next_header: IpProtocol::Udp,
                   hop_limit: 64,
                   src: Ipv6Addr::new(0x2001, 0xdb8, 0, 1, 0, 0, 0, 1),
                   dst: Ipv6Addr::new(0x2001, 0xdb8, 0, 1, 0, 0, 0, 2),
               }));
}

#[test]
fn peel_success_ipv6_in_ipv6() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(PACKET_ETH_IPV6_IPV6, vec![]).result;
    assert_eq!(result.len(), 3);
}

#[test]
fn peel_success_ipv4_in_ipv4() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let result = peel.traverse(PACKET_ETH_IPV4_IPV4, vec![]).result;
    assert_eq!(result.len(), 3);
}

#[test]
fn peel_success_eth() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let mut input = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00];
    input.extend_from_slice(&[0xff; 500]);
    let result = peel.traverse(&input, vec![]).result;
    assert_eq!(result.len(), 1);
}

#[test]
fn peel_failure_path_tracking() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let mut result = peel.traverse(PACKET_ETH_IPV4_TCP, vec![]).result;
    result.swap(0, 1);
    assert!(track_connection(Some(peel.data.as_mut().unwrap()), Some(&result), 0, 0).is_ok());
    assert_eq!(peel.data.as_mut().unwrap().connection_count(), 1);
}

#[test]
fn peel_track_timeout() {
    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    peel.data.as_mut().unwrap().timeout = Duration::from_std(std::time::Duration::from_millis(1)).unwrap();

    let result = peel.traverse(PACKET_ETH_IPV4_TCP, vec![]).result;
    assert_eq!(result.len(), 3);
    std::thread::sleep(std::time::Duration::from_millis(10));

    let result = peel.traverse(PACKET_ETH_IPV4_TCP, vec![]).result;
    assert_eq!(result.len(), 3);
}
