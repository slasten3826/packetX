use x12_server::compat::x11::X11Request;
use x12_server::compat::x11_wire::{parse_request_bytes, X11ErrorCode};

fn header(opcode: u8, data: u8, length_units: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity((length_units as usize) * 4);
    out.push(opcode);
    out.push(data);
    out.extend_from_slice(&length_units.to_le_bytes());
    out
}

#[test]
fn parse_create_window_request_into_internal_claim() {
    let mut request = header(1, 0, 8);
    request.extend_from_slice(&77u32.to_le_bytes());
    request.extend_from_slice(&1u32.to_le_bytes());
    request.extend_from_slice(&10i16.to_le_bytes());
    request.extend_from_slice(&20i16.to_le_bytes());
    request.extend_from_slice(&640u16.to_le_bytes());
    request.extend_from_slice(&480u16.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&1u16.to_le_bytes());
    request.extend_from_slice(&0u32.to_le_bytes());
    request.extend_from_slice(&0u32.to_le_bytes());

    let parsed = parse_request_bytes(&request).expect("request should parse");
    assert_eq!(
        parsed,
        X11Request::CreateWindow {
            id: 77,
            parent: 1,
            x: 10,
            y: 20,
            width: 640,
            height: 480
        }
    );
}

#[test]
fn parse_configure_window_partial_update() {
    let mut request = header(12, 0, 5);
    request.extend_from_slice(&77u32.to_le_bytes());
    request.extend_from_slice(&0b0101u16.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&30u32.to_le_bytes());
    request.extend_from_slice(&800u32.to_le_bytes());

    let parsed = parse_request_bytes(&request).expect("request should parse");
    assert_eq!(
        parsed,
        X11Request::ConfigureWindow {
            id: 77,
            x: Some(30),
            y: None,
            width: Some(800),
            height: None
        }
    );
}

#[test]
fn unsupported_opcode_returns_bad_request() {
    let request = header(99, 0, 1);
    let err = parse_request_bytes(&request).expect_err("opcode should fail");
    assert_eq!(err.code, X11ErrorCode::BadRequest);
}

#[test]
fn unsupported_configure_bits_return_bad_match() {
    let mut request = header(12, 0, 4);
    request.extend_from_slice(&77u32.to_le_bytes());
    request.extend_from_slice(&0b1_0000u16.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&3u32.to_le_bytes());

    let err = parse_request_bytes(&request).expect_err("mask should fail");
    assert_eq!(err.code, X11ErrorCode::BadMatch);
}

#[test]
fn malformed_length_returns_bad_length() {
    let mut request = header(8, 0, 3);
    request.extend_from_slice(&77u32.to_le_bytes());

    let err = parse_request_bytes(&request).expect_err("length should fail");
    assert_eq!(err.code, X11ErrorCode::BadLength);
}
