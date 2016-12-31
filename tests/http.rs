extern crate nom;
extern crate peel_ip;
use peel_ip::prelude::*;

#[test]
fn parse_http_request_success_get() {
    let mut parser = HttpParser;
    let res = parser.parse(b"GET /some/path/ HTTP/1.0\r\nHost: myhost.com\r\nUser-agent: Myagent/0.1\r\n\r\nTest data",
               None)
        .unwrap()
        .1;
    println!("{}", res);
    assert_eq!(res,
               Layer::Http(HttpPacket::Request(HttpRequest {
                   request_method: HttpRequestMethod::Get,
                   path: "/some/path/".to_owned(),
                   version: HttpVersion {
                       major: 1,
                       minor: 0,
                   },
                   headers: vec![HttpHeader {
                                     key: "Host".to_owned(),
                                     value: "myhost.com".to_owned(),
                                 },
                                 HttpHeader {
                                     key: "User-agent".to_owned(),
                                     value: "Myagent/0.1".to_owned(),
                                 }],
               })));
}

#[test]
fn parse_http_request_success_post() {
    let mut parser = HttpParser;
    let res = parser.parse(b"POST / HTTP/1.1\r\nHost: abc.com\r\n\r\n",
               None)
        .unwrap()
        .1;
    assert_eq!(res,
               Layer::Http(HttpPacket::Request(HttpRequest {
                   request_method: HttpRequestMethod::Post,
                   path: "/".to_owned(),
                   version: HttpVersion {
                       major: 1,
                       minor: 1,
                   },
                   headers: vec![HttpHeader {
                                     key: "Host".to_owned(),
                                     value: "abc.com".to_owned(),
                                 }],
               })));
}

#[test]
fn parse_http_request_success_methods() {
    let mut parser = HttpParser;
    let header = " / HTTP/1.1\r\nHost: abc.com\r\n\r\n";
    let methods = ["GET", "POST", "HEAD", "PUT", "DELETE", "TRACE", "OPTIONS", "CONNECT", "PATCH"];

    for method in &methods {
        let mut input = Vec::from(*method);
        input.extend_from_slice(header.as_bytes());
        parser.parse(&input, None).unwrap();
    }
}

#[test]
fn parse_http_request_failure_wrong_method() {
    let mut parser = HttpParser;
    assert_eq!(parser.parse(b"GET", None),
               IResult::Incomplete(Needed::Size(4)));

    let input = b"GOT ";
    assert_eq!(parser.parse(input, None),
               IResult::Error(Err::Position(ErrorKind::Alt, &input[..])));
}

#[test]
fn parse_http_request_failure_wrong_path() {
    let mut parser = HttpParser;
    let input = b"GET HTTP/1.1";
    assert_eq!(parser.parse(input, None),
               IResult::Error(Err::Position(ErrorKind::Alt, &input[..])));
}

#[test]
fn parse_http_request_failure_wrong_version() {
    let mut parser = HttpParser;
    let input = b"GET / HTTP/1.k";
    assert_eq!(parser.parse(input, None),
               IResult::Error(Err::Position(ErrorKind::Alt, &input[..])));
}

#[test]
fn parse_http_request_failure_too_small() {
    let mut parser = HttpParser;
    let input = b"GET / HTTP/1.";
    assert_eq!(parser.parse(input, None),
               IResult::Incomplete(Needed::Unknown));
}

#[test]
fn parse_http_response_success_moved() {
    let mut parser = HttpParser;
    let res = parser.parse(b"HTTP/1.1 301 Moved Permanently\r\nLocation: https://facebook.com\r\n\r\n",
               None)
        .unwrap()
        .1;
    assert_eq!(res,
               Layer::Http(HttpPacket::Response(HttpResponse {
                   version: HttpVersion {
                       major: 1,
                       minor: 1,
                   },
                   code: 301,
                   reason: "Moved Permanently".to_owned(),
                   headers: vec![HttpHeader {
                                     key: "Location".to_owned(),
                                     value: "https://facebook.com".to_owned(),
                                 }],
               })));
}

#[test]
fn parse_http_response_success_ok() {
    let mut parser = HttpParser;
    let res = parser.parse(b"HTTP/1.0 200 OK\r\nHost: abc.com\r\n\r\n",
               None)
        .unwrap()
        .1;
    assert_eq!(res,
               Layer::Http(HttpPacket::Response(HttpResponse {
                   version: HttpVersion {
                       major: 1,
                       minor: 0,
                   },
                   code: 200,
                   reason: "OK".to_owned(),
                   headers: vec![HttpHeader {
                                     key: "Host".to_owned(),
                                     value: "abc.com".to_owned(),
                                 }],
               })));
}

#[test]
fn parse_http_response_failure_wrong_protocol() {
    let mut parser = HttpParser;
    let input = b"HTTk/1.1";
    assert_eq!(parser.parse(input, None),
               IResult::Error(Err::Position(ErrorKind::Alt, &input[..])));
}

#[test]
fn parse_http_response_failure_too_small() {
    let mut parser = HttpParser;
    let input = b"HTTP/1.1";
    assert_eq!(parser.parse(input, None),
               IResult::Incomplete(Needed::Size(9)));
}

#[test]
fn parse_http_response_failure_wrong_status_code() {
    let mut parser = HttpParser;
    let input = b"HTTP/1.1 20A OK\r\n";
    assert_eq!(parser.parse(input, None),
               IResult::Error(Err::Position(ErrorKind::Alt, &input[..])));
}
