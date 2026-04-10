use x12_server::compat::x11_wire::{
    process_session_wire_request, ClientSession, X11ErrorCode,
};
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

fn resource_request(opcode: u8, id: u32) -> Vec<u8> {
    let mut request = Vec::with_capacity(8);
    request.push(opcode);
    request.push(0);
    request.extend_from_slice(&2u16.to_le_bytes());
    request.extend_from_slice(&id.to_le_bytes());
    request
}

fn configure_window_request(id: u32, x: Option<i16>, width: Option<u16>) -> Vec<u8> {
    let mut mask = 0u16;
    let mut values = Vec::new();

    if let Some(x) = x {
        mask |= 1 << 0;
        values.extend_from_slice(&(x as i32 as u32).to_le_bytes());
    }
    if let Some(width) = width {
        mask |= 1 << 2;
        values.extend_from_slice(&(width as u32).to_le_bytes());
    }

    let length_units = 3 + (values.len() / 4) as u16;
    let mut request = Vec::with_capacity(length_units as usize * 4);
    request.push(12);
    request.push(0);
    request.extend_from_slice(&length_units.to_le_bytes());
    request.extend_from_slice(&id.to_le_bytes());
    request.extend_from_slice(&mask.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&values);
    request
}

#[test]
fn session_processes_multiple_requests_with_incrementing_sequence() {
    let mut server = ServerState::new();
    let mut session = ClientSession::new(1);

    let (seq1, create_snapshot) = process_session_wire_request(
        &mut server,
        &mut session,
        &create_window_request(0x0020_004d, 1, 10, 20, 640, 480),
    )
    .expect("create should succeed");
    let (seq2, map_snapshot) = process_session_wire_request(
        &mut server,
        &mut session,
        &resource_request(8, 0x0020_004d),
    )
    .expect("map should succeed");
    let (seq3, configure_snapshot) = process_session_wire_request(
        &mut server,
        &mut session,
        &configure_window_request(0x0020_004d, Some(30), Some(800)),
    )
    .expect("configure should succeed");

    assert_eq!(seq1, 1);
    assert_eq!(seq2, 2);
    assert_eq!(seq3, 3);
    assert!(create_snapshot.contains("id: 2097229"));
    assert!(map_snapshot.contains("forms_mapped: 1"));
    assert!(configure_snapshot.contains("size: [800, 480]"));
}

#[test]
fn session_rejects_xid_outside_assigned_range() {
    let mut server = ServerState::new();
    let mut session = ClientSession::new(1);

    let error = process_session_wire_request(
        &mut server,
        &mut session,
        &create_window_request(77, 1, 10, 20, 640, 480),
    )
    .expect_err("out-of-range xid should fail");

    assert_eq!(error[1], X11ErrorCode::BadValue as u8);
    assert_eq!(u16::from_le_bytes([error[2], error[3]]), 1);
}

#[test]
fn session_rejects_reference_to_unowned_window() {
    let mut server = ServerState::new();
    let mut session = ClientSession::new(1);

    let error = process_session_wire_request(
        &mut server,
        &mut session,
        &resource_request(8, 0x0020_004d),
    )
    .expect_err("mapping unknown local xid should fail");

    assert_eq!(error[1], X11ErrorCode::BadWindow as u8);
    assert_eq!(u16::from_le_bytes([error[2], error[3]]), 1);
}
