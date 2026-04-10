use x12_server::compat::x11_wire::{handle_setup_bytes, X11SetupOutcome};

fn setup_request(byte_order: u8, major: u16, minor: u16, auth_proto_len: u16, auth_data_len: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(12);
    out.push(byte_order);
    out.push(0);
    out.extend_from_slice(&major.to_le_bytes());
    out.extend_from_slice(&minor.to_le_bytes());
    out.extend_from_slice(&auth_proto_len.to_le_bytes());
    out.extend_from_slice(&auth_data_len.to_le_bytes());
    out.extend_from_slice(&[0u8; 2]);
    out
}

#[test]
fn little_endian_empty_auth_gets_setup_success() {
    let (outcome, response) =
        handle_setup_bytes(&setup_request(b'l', 11, 0, 0, 0)).expect("server should answer");

    assert_eq!(
        outcome,
        X11SetupOutcome::Success {
            vendor: "packetX",
            root_window: 1
        }
    );
    assert_eq!(response[0], 1);
    assert_eq!(u16::from_le_bytes([response[2], response[3]]), 11);
    assert_eq!(u16::from_le_bytes([response[4], response[5]]), 0);
    assert!(response.windows(b"packetX".len()).any(|window| window == b"packetX"));
}

#[test]
fn big_endian_client_is_rejected() {
    let (outcome, response) =
        handle_setup_bytes(&setup_request(b'B', 11, 0, 0, 0)).expect("server should answer");

    assert_eq!(response[0], 0);
    match outcome {
        X11SetupOutcome::Failed { reason } => {
            assert!(reason.contains("little-endian"));
        }
        other => panic!("expected failure, got {other:?}"),
    }
}

#[test]
fn non_empty_auth_is_rejected() {
    let mut request = setup_request(b'l', 11, 0, 4, 4);
    request.extend_from_slice(b"MIT\0");
    request.extend_from_slice(b"auth");
    let (outcome, response) = handle_setup_bytes(&request).expect("server should answer");

    assert_eq!(response[0], 0);
    match outcome {
        X11SetupOutcome::Failed { reason } => {
            assert!(reason.contains("authorization"));
        }
        other => panic!("expected failure, got {other:?}"),
    }
}
