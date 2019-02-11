//! Internet Protocol version 6 related packet processing
use prelude::*;

/// The IPv6 parser
#[derive(Debug)]
pub struct Ipv6Parser;

impl Parsable<PathIp> for Ipv6Parser {
    /// Parse an `Ipv6Packet` from an `&[u8]`
    fn parse<'a>(
        &mut self,
        input: &'a [u8],
        result: Option<&ParserResultVec>,
        _: Option<&mut PathIp>,
    ) -> IResult<&'a [u8], ParserResult> {
        do_parse!(
            input,
            // Check the type from the parent parser (Ethernet)
            expr_opt!(match result {
                Some(vector) => match vector.last() {

                    // Check the correct EtherType or IP encapsulation
                    Some(ref any) => match (any.downcast_ref::<EthernetPacket>(),
                                            any.downcast_ref::<Ipv4Packet>(),
                                            any.downcast_ref::<Ipv6Packet>()) {


                        // Ethernet
                        (Some(eth), _, _) => if eth.ethertype == EtherType::Ipv6 {
                            Some(())
                        } else {
                            None
                        },

                        // IPv6 in IPv4
                        (_, Some(ipv4), _) => if ipv4.protocol == IpProtocol::Ipv6 {
                            Some(())
                        } else {
                            None
                        },

                        // IPv6 in IPv6
                        (_, _, Some(ipv6)) => if ipv6.next_header == IpProtocol::Ipv6 {
                            Some(())
                        } else {
                            None
                        },

                        _ => None,
                    },

                    // Previous result found, but not correct parent
                    _ => None,
                },
                // Parse also if no result is given, for testability
                None => Some(()),
            }) >>

            // Parse the actual packet
            ver_tc_fl: bits!(tuple!(tag_bits!(u8, 4, 6),
                                    take_bits!(u8, 8),
                                    take_bits!(u32, 20))) >>
            payload_length: be_u16 >>
            next_header: map_opt!(be_u8, IpProtocol::from_u8) >>
            hop_limit: be_u8 >>
            src: tuple!(be_u16, be_u16, be_u16, be_u16, be_u16, be_u16, be_u16, be_u16) >>
            dst: tuple!(be_u16, be_u16, be_u16, be_u16, be_u16, be_u16, be_u16, be_u16) >>

            (Box::new(Ipv6Packet {
                version: ver_tc_fl.0,
                traffic_class: ver_tc_fl.1,
                flow_label: ver_tc_fl.2,
                payload_length: payload_length,
                next_header: next_header,
                hop_limit: hop_limit,
                src: Ipv6Addr::new(src.0, src.1, src.2, src.3,
                                   src.4, src.5, src.6, src.7),
                dst: Ipv6Addr::new(dst.0, dst.1, dst.2, dst.3,
                                   dst.4, dst.5, dst.6, dst.7),
            }))
        )
    }
}

impl fmt::Display for Ipv6Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IPv6")
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Representation of an Internet Protocol version 6 packet
pub struct Ipv6Packet {
    /// The constant 6 (bit sequence 0110).
    pub version: u8,

    /// The bits of this field hold two values. The 6 most-significant bits are used for
    /// differentiated services, which is used to classify packets. The remaining two bits are used
    /// for ECN; priority values subdivide into ranges: traffic where the source provides
    /// congestion control and non-congestion control traffic.
    pub traffic_class: u8,

    /// Originally created for giving real-time applications special service. The flow label when
    /// set to a non-zero value now serves as a hint to routers and switches with multiple outbound
    /// paths that these packets should stay on the same path so that they will not be reordered.
    /// It has further been suggested that the flow label be used to help detect spoofed packets.
    pub flow_label: u32,

    /// The size of the payload in octets, including any extension headers. The length is set to
    /// zero when a Hop-by-Hop extension header carries a Jumbo Payload option.
    pub payload_length: u16,

    /// Specifies the type of the next header. This field usually specifies the transport layer
    /// protocol used by a packet's payload. When extension headers are present in the packet this
    /// field indicates which extension header follows. The values are shared with those used for
    /// the IPv4 protocol field, as both fields have the same function.
    pub next_header: IpProtocol,

    /// Replaces the time to live field of IPv4. This value is decremented by one at each
    /// intermediate node visited by the packet. When the counter reaches 0 the packet is
    /// discarded.
    pub hop_limit: u8,

    /// Source address
    pub src: Ipv6Addr,

    /// Destination address
    pub dst: Ipv6Addr,
}
