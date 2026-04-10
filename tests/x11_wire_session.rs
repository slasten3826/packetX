use x12_server::compat::x11::X11Request;
use x12_server::compat::x11_wire::{process_session_wire_request, ClientSession, X11ErrorCode};
use x12_server::process_request_for_session;
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

#[test]
fn session_rejects_duplicate_xid_allocation() {
    let mut server = ServerState::new();
    let mut session = ClientSession::new(1);
    let request = create_window_request(0x0020_004d, 1, 10, 20, 640, 480);

    let _ = process_session_wire_request(&mut server, &mut session, &request)
        .expect("first create should succeed");
    let error = process_session_wire_request(&mut server, &mut session, &request)
        .expect_err("duplicate xid should fail");

    assert_eq!(error[1], X11ErrorCode::BadValue as u8);
    assert_eq!(u16::from_le_bytes([error[2], error[3]]), 2);
}

#[test]
fn session_continues_after_protocol_error() {
    let mut server = ServerState::new();
    let mut session = ClientSession::new(1);

    let bad = process_session_wire_request(&mut server, &mut session, &resource_request(8, 0x0020_004d))
        .expect_err("first map without ownership should fail");
    let (seq_create, _) = process_session_wire_request(
        &mut server,
        &mut session,
        &create_window_request(0x0020_004d, 1, 10, 20, 640, 480),
    )
    .expect("session should continue after error");
    let (seq_map, snapshot) = process_session_wire_request(
        &mut server,
        &mut session,
        &resource_request(8, 0x0020_004d),
    )
    .expect("map should succeed after create");

    assert_eq!(u16::from_le_bytes([bad[2], bad[3]]), 1);
    assert_eq!(seq_create, 2);
    assert_eq!(seq_map, 3);
    assert!(snapshot.contains("forms_mapped: 1"));
}

#[test]
fn different_sessions_get_distinct_xid_bases() {
    let first = ClientSession::new(1);
    let second = ClientSession::new(2);

    assert_ne!(first.xid_base, second.xid_base);
    assert_eq!(first.xid_mask, second.xid_mask);
    assert!(first.owns_xid(first.xid_base | 0x004d));
    assert!(second.owns_xid(second.xid_base | 0x004d));
    assert!(!first.owns_xid(second.xid_base | 0x004d));
}

#[test]
fn server_records_form_owner_session() {
    let mut server = ServerState::new();

    let _ = process_request_for_session(
        &mut server,
        7,
        &X11Request::CreateWindow {
            id: 0x00e0_004d,
            parent: 1,
            x: 10,
            y: 20,
            width: 320,
            height: 240,
        },
    );

    let form = server.form(0x00e0_004d).expect("form should exist");
    assert_eq!(form.owner_session_id, 7);
    assert_eq!(server.forms_for_session(7).count(), 1);
}

#[test]
fn separate_sessions_can_allocate_same_low_bits_without_collision() {
    let mut server = ServerState::new();
    let mut first = ClientSession::new(1);
    let mut second = ClientSession::new(2);

    let first_id = first.xid_base | 0x004d;
    let second_id = second.xid_base | 0x004d;

    process_session_wire_request(
        &mut server,
        &mut first,
        &create_window_request(first_id, 1, 10, 20, 320, 240),
    )
    .expect("first client should allocate its xid");
    process_session_wire_request(
        &mut server,
        &mut second,
        &create_window_request(second_id, 1, 30, 40, 320, 240),
    )
    .expect("second client should allocate its xid");

    assert_eq!(server.forms.len(), 2);
    assert_eq!(server.form(first_id).expect("first form").owner_session_id, 1);
    assert_eq!(server.form(second_id).expect("second form").owner_session_id, 2);
}

#[test]
fn client_session_try_new_returns_none_when_xid_space_is_exhausted() {
    assert!(ClientSession::try_new(2047).is_some());
    assert!(ClientSession::try_new(2048).is_none());
}

#[test]
fn cleanup_session_removes_client_and_owned_forms() {
    let mut server = ServerState::new();
    assert!(server.register_client(11, 0x0160_0000, 0x001f_ffff));
    assert!(server.register_client(12, 0x0180_0000, 0x001f_ffff));

    let _ = process_request_for_session(
        &mut server,
        11,
        &X11Request::CreateWindow {
            id: 0x0160_004d,
            parent: 1,
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        },
    );
    let _ = process_request_for_session(
        &mut server,
        12,
        &X11Request::CreateWindow {
            id: 0x0180_004d,
            parent: 1,
            x: 10,
            y: 10,
            width: 120,
            height: 120,
        },
    );

    server.cleanup_session(11);

    assert!(server.client(11).is_none());
    assert!(server.client(12).is_some());
    assert!(server.form(0x0160_004d).is_none());
    assert!(server.form(0x0180_004d).is_some());
    assert_eq!(server.forms.len(), 1);
    assert_eq!(server.form(0x0180_004d).expect("remaining form").stacking_rank, 0);
}
