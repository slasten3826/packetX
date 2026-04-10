use x12_server::compat::x11_wire::process_wire_request;
use x12_server::server::ServerState;

fn create_window_request(id: u32, parent: u32, x: i16, y: i16, width: u16, height: u16) -> Vec<u8> {
    let mut request = Vec::with_capacity(32);
    request.push(1);
    request.push(0);
    request.extend_from_slice(&8u16.to_le_bytes());
    request.extend_from_slice(&id.to_le_bytes());
    request.extend_from_slice(&parent.to_le_bytes());
    request.extend_from_slice(&x.to_le_bytes());
    request.extend_from_slice(&y.to_le_bytes());
    request.extend_from_slice(&width.to_le_bytes());
    request.extend_from_slice(&height.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&1u16.to_le_bytes());
    request.extend_from_slice(&0u32.to_le_bytes());
    request.extend_from_slice(&0u32.to_le_bytes());
    request
}

#[test]
fn valid_wire_create_window_reaches_existing_spine() {
    let mut server = ServerState::new();
    let request = create_window_request(77, 1, 10, 20, 640, 480);

    let snapshot = process_wire_request(&mut server, 1, &request).expect("request should succeed");

    assert!(snapshot.contains("forms_total: 1"));
    assert!(snapshot.contains("id: 77"));
    assert!(snapshot.contains("status: Killed"));
}

#[test]
fn invalid_wire_request_returns_xerror_bytes() {
    let mut server = ServerState::new();
    let mut request = vec![99, 0];
    request.extend_from_slice(&1u16.to_le_bytes());

    let error = process_wire_request(&mut server, 7, &request).expect_err("request should fail");

    assert_eq!(error.len(), 32);
    assert_eq!(error[0], 0);
    assert_eq!(error[1], 1);
    assert_eq!(u16::from_le_bytes([error[2], error[3]]), 7);
    assert_eq!(error[10], 99);
}
