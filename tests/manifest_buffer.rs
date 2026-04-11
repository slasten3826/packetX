use x12_server::compat::x11::X11Request;
use x12_server::manifest::{render_manifest_buffer, ManifestBuffer};
use x12_server::process_request;
use x12_server::server::{PacketAtom, PacketOrigin, ProcessKind, ServerState};

fn run(server: &mut ServerState, request: &X11Request) {
    let _ = process_request(server, request);
}

#[test]
fn manifest_buffer_dimensions_are_preserved() {
    let buffer = ManifestBuffer::new(320, 240);

    assert_eq!(buffer.width(), 320);
    assert_eq!(buffer.height(), 240);
    assert_eq!(buffer.get_pixel(0, 0), Some(0));
    assert_eq!(buffer.get_pixel(319, 239), Some(0));
    assert_eq!(buffer.get_pixel(320, 0), None);
}

#[test]
fn manifest_buffer_encodes_valid_ppm_header_and_rgb_payload() {
    let mut buffer = ManifestBuffer::new(2, 1);
    buffer.clear(0);
    buffer.fill_rect(0, 0, 1, 1, 0xff11_2233);
    buffer.fill_rect(1, 0, 1, 1, 0xff44_5566);

    let ppm = buffer.encode_ppm();
    let header = b"P6\n2 1\n255\n";

    assert!(ppm.starts_with(header));
    assert_eq!(&ppm[header.len()..], &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
}

#[test]
fn fill_rect_is_clipped_to_buffer_bounds() {
    let mut buffer = ManifestBuffer::new(4, 4);
    buffer.clear(0);
    buffer.fill_rect(-1, -1, 3, 3, 0xff00_ff00);

    assert_eq!(buffer.get_pixel(0, 0), Some(0xff00_ff00));
    assert_eq!(buffer.get_pixel(1, 1), Some(0xff00_ff00));
    assert_eq!(buffer.get_pixel(2, 2), Some(0));
    assert_eq!(buffer.get_pixel(3, 3), Some(0));
}

#[test]
fn single_mapped_form_appears_in_manifest_buffer() {
    let mut server = ServerState::new();

    run(
        &mut server,
        &X11Request::CreateWindow {
            id: 1,
            parent: 0,
            x: 2,
            y: 3,
            width: 4,
            height: 2,
        },
    );
    run(&mut server, &X11Request::MapWindow { id: 1 });

    let buffer = render_manifest_buffer(&server.forms, 16, 16);
    let inside = buffer.get_pixel(2, 3).expect("inside pixel should exist");
    let outside = buffer.get_pixel(0, 0).expect("background pixel should exist");

    assert_ne!(inside, outside);
    assert_eq!(buffer.get_pixel(5, 4), Some(inside));
    assert_eq!(buffer.get_pixel(6, 5), Some(outside));
}

#[test]
fn upper_form_overwrites_lower_in_overlap_region() {
    let mut server = ServerState::new();

    run(
        &mut server,
        &X11Request::CreateWindow {
            id: 1,
            parent: 0,
            x: 0,
            y: 0,
            width: 6,
            height: 6,
        },
    );
    run(&mut server, &X11Request::MapWindow { id: 1 });
    run(
        &mut server,
        &X11Request::CreateWindow {
            id: 2,
            parent: 0,
            x: 2,
            y: 2,
            width: 4,
            height: 4,
        },
    );
    run(&mut server, &X11Request::MapWindow { id: 2 });

    let buffer = render_manifest_buffer(&server.forms, 16, 16);
    let lower_only = buffer.get_pixel(1, 1).expect("lower-only pixel should exist");
    let overlap = buffer.get_pixel(3, 3).expect("overlap pixel should exist");
    let upper_only = buffer.get_pixel(5, 5).expect("upper-only pixel should exist");

    assert_eq!(overlap, upper_only);
    assert_ne!(overlap, lower_only);
}

#[test]
fn fully_occluded_form_is_not_composited_into_manifest_buffer() {
    let mut server = ServerState::new();

    run(
        &mut server,
        &X11Request::CreateWindow {
            id: 21,
            parent: 0,
            x: 0,
            y: 0,
            width: 6,
            height: 6,
        },
    );
    run(&mut server, &X11Request::MapWindow { id: 21 });
    run(
        &mut server,
        &X11Request::CreateWindow {
            id: 22,
            parent: 0,
            x: 0,
            y: 0,
            width: 6,
            height: 6,
        },
    );
    run(&mut server, &X11Request::MapWindow { id: 22 });

    let buffer = render_manifest_buffer(&server.forms, 16, 16);
    let overlap = buffer.get_pixel(2, 2).expect("overlap pixel should exist");
    let upper_only = render_manifest_buffer(&{
        let mut isolated = ServerState::new();
        run(
            &mut isolated,
            &X11Request::CreateWindow {
                id: 22,
                parent: 0,
                x: 0,
                y: 0,
                width: 6,
                height: 6,
            },
        );
        run(&mut isolated, &X11Request::MapWindow { id: 22 });
        isolated.forms
    }, 16, 16)
    .get_pixel(2, 2)
    .expect("upper-only pixel should exist");

    assert_eq!(overlap, upper_only);
    assert_eq!(server.form(21).expect("lower form").visible_area, 0);
    assert_eq!(server.form(21).expect("lower form").occluded_by, Some(22));
}

#[test]
fn unmapped_form_does_not_appear_in_manifest_buffer() {
    let mut server = ServerState::new();

    run(
        &mut server,
        &X11Request::CreateWindow {
            id: 9,
            parent: 0,
            x: 1,
            y: 1,
            width: 5,
            height: 5,
        },
    );

    let buffer = render_manifest_buffer(&server.forms, 16, 16);
    let background = buffer.get_pixel(0, 0).expect("background pixel should exist");

    assert_eq!(buffer.get_pixel(2, 2), Some(background));
}

#[test]
fn manifest_state_tracks_damage_for_first_mapped_form() {
    let mut server = ServerState::new();

    run(
        &mut server,
        &X11Request::CreateWindow {
            id: 11,
            parent: 0,
            x: 4,
            y: 5,
            width: 6,
            height: 3,
        },
    );
    run(&mut server, &X11Request::MapWindow { id: 11 });

    let damage = server.manifest_state.damage();
    assert_eq!(damage.len(), 1);
    assert_eq!(damage[0].x, 4);
    assert_eq!(damage[0].y, 5);
    assert_eq!(damage[0].width, 6);
    assert_eq!(damage[0].height, 3);
}

#[test]
fn manifest_front_buffer_returns_to_background_after_unmap() {
    let mut server = ServerState::new();

    run(
        &mut server,
        &X11Request::CreateWindow {
            id: 12,
            parent: 0,
            x: 1,
            y: 1,
            width: 4,
            height: 4,
        },
    );
    run(&mut server, &X11Request::MapWindow { id: 12 });
    let colored = server
        .manifest_state
        .front()
        .get_pixel(2, 2)
        .expect("pixel should exist after map");

    run(&mut server, &X11Request::UnmapWindow { id: 12 });

    let background = server
        .manifest_state
        .front()
        .get_pixel(0, 0)
        .expect("background pixel should exist");
    let cleared = server
        .manifest_state
        .front()
        .get_pixel(2, 2)
        .expect("pixel should exist after unmap");

    assert_ne!(colored, background);
    assert_eq!(cleared, background);
    assert_eq!(server.manifest_state.damage().len(), 1);
}

#[test]
fn manifest_skips_damage_diff_on_clean_state() {
    let mut server = ServerState::new();

    run(
        &mut server,
        &X11Request::CreateWindow {
            id: 13,
            parent: 0,
            x: 2,
            y: 2,
            width: 3,
            height: 3,
        },
    );
    run(&mut server, &X11Request::MapWindow { id: 13 });
    assert_eq!(server.manifest_state.damage().len(), 1);

    let _ = x12_server::manifest::emit_snapshot(
        &mut server,
        &mut x12_server::server::PacketAtom::new(
            999,
            999,
            x12_server::server::PacketOrigin::X11Client,
            x12_server::server::ProcessKind::Scene,
        ),
    );

    assert!(server.manifest_state.damage().is_empty());
}

#[test]
fn cleanup_session_marks_manifest_dirty_and_clears_removed_form() {
    let mut server = ServerState::new();
    assert!(server.register_client(41, 0x0520_0000, 0x001f_ffff));

    let _ = x12_server::process_request_for_session(
        &mut server,
        41,
        &X11Request::CreateWindow {
            id: 0x0520_004d,
            parent: 1,
            x: 3,
            y: 3,
            width: 5,
            height: 5,
        },
    );
    let _ = x12_server::process_request_for_session(
        &mut server,
        41,
        &X11Request::MapWindow { id: 0x0520_004d },
    );
    assert_eq!(server.manifest_state.damage().len(), 1);

    let _ = x12_server::manifest::emit_snapshot(
        &mut server,
        &mut PacketAtom::new(1001, 1001, PacketOrigin::X11Client, ProcessKind::Scene),
    );
    assert!(server.manifest_state.damage().is_empty());

    server.cleanup_session(41);

    let snapshot = x12_server::manifest::emit_snapshot(
        &mut server,
        &mut PacketAtom::new(1002, 1002, PacketOrigin::X11Client, ProcessKind::Scene),
    );

    assert!(snapshot.contains("damage_rects: 1"));
    let background = server
        .manifest_state
        .front()
        .get_pixel(3, 3)
        .expect("background pixel should exist after cleanup");
    assert_eq!(server.forms.len(), 0);
    assert_eq!(background, server.manifest_state.front().get_pixel(0, 0).unwrap());
}

#[test]
fn killed_packet_snapshot_clears_manifest_dirty_state() {
    let mut server = ServerState::new();
    assert!(server.register_client(51, 0x0660_0000, 0x001f_ffff));
    assert!(server.register_client(52, 0x0680_0000, 0x001f_ffff));

    let _ = x12_server::process_request_for_session(
        &mut server,
        51,
        &X11Request::CreateWindow {
            id: 0x0660_004d,
            parent: 1,
            x: 4,
            y: 4,
            width: 6,
            height: 6,
        },
    );
    let _ = x12_server::process_request_for_session(
        &mut server,
        51,
        &X11Request::MapWindow { id: 0x0660_004d },
    );
    let _ = x12_server::manifest::emit_snapshot(
        &mut server,
        &mut PacketAtom::new(1101, 1101, PacketOrigin::X11Client, ProcessKind::Scene),
    );
    assert!(server.manifest_state.damage().is_empty());

    let _ = x12_server::process_request_for_session(
        &mut server,
        52,
        &X11Request::ConfigureWindow {
            id: 0x0660_004d,
            x: Some(20),
            y: None,
            width: None,
            height: None,
        },
    );

    assert!(server.manifest_state.damage().is_empty());
}
