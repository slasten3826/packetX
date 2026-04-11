use x12_server::compat::x11::X11Request;
use x12_server::manifest::{render_manifest_buffer, ManifestBuffer};
use x12_server::process_request;
use x12_server::server::ServerState;

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

    let buffer = render_manifest_buffer(&server, 16, 16);
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

    let buffer = render_manifest_buffer(&server, 16, 16);
    let lower_only = buffer.get_pixel(1, 1).expect("lower-only pixel should exist");
    let overlap = buffer.get_pixel(3, 3).expect("overlap pixel should exist");
    let upper_only = buffer.get_pixel(5, 5).expect("upper-only pixel should exist");

    assert_eq!(overlap, upper_only);
    assert_ne!(overlap, lower_only);
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

    let buffer = render_manifest_buffer(&server, 16, 16);
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
