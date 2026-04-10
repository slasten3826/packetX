use x12_server::compat::x11::{scenarios, X11Request};
use x12_server::process_request;
use x12_server::server::ServerState;

fn run(server: &mut ServerState, request: &X11Request) -> String {
    process_request(server, request)
}

#[test]
fn fully_occluded_lower_form_is_killed_in_crystall() {
    let mut server = ServerState::new();

    for request in scenarios::full_occlusion_sequence() {
        let _ = run(&mut server, &request);
    }

    let lower = server.form(1).expect("lower form should exist");
    let upper = server.form(2).expect("upper form should exist");

    assert_eq!(lower.visible_area, 0);
    assert!(!lower.visible);
    assert_eq!(lower.occluded_by, Some(2));
    assert_eq!(upper.visible_area, upper.total_area);
    assert!(upper.visible);
}

#[test]
fn partial_overlap_reduces_visible_area_without_killing_form() {
    let mut server = ServerState::new();

    for request in scenarios::partial_overlap_sequence() {
        let _ = run(&mut server, &request);
    }

    let lower = server.form(1).expect("lower form should exist");
    let upper = server.form(2).expect("upper form should exist");

    assert_eq!(lower.total_area, 10_000);
    assert_eq!(lower.visible_area, 5_000);
    assert!(lower.visible);
    assert_eq!(lower.occluded_by, None);
    assert_eq!(upper.visible_area, upper.total_area);
}

#[test]
fn unmap_restores_lower_form_visibility() {
    let mut server = ServerState::new();
    let mut last_snapshot = String::new();

    for request in scenarios::unmap_restore_sequence() {
        last_snapshot = run(&mut server, &request);
    }

    let lower = server.form(1).expect("lower form should exist");
    let upper = server.form(2).expect("upper form should exist");

    assert_eq!(lower.visible_area, lower.total_area);
    assert!(lower.visible);
    assert!(!upper.mapped);
    assert_eq!(upper.visible_area, 0);
    assert!(
        last_snapshot.contains("forms_visible: 1"),
        "final snapshot should show only one visible form"
    );
}

#[test]
fn named_scenarios_are_exposed_for_cli_harness() {
    for name in scenarios::NAMES {
        let sequence = scenarios::named(name);
        assert!(sequence.is_some(), "scenario {name} should resolve");
        assert!(
            !sequence.expect("scenario should exist").is_empty(),
            "scenario {name} should not be empty"
        );
    }
}

#[test]
fn unmapped_forms_do_not_contribute_to_manifest_pressure() {
    let mut server = ServerState::new();

    let snapshot = run(
        &mut server,
        &X11Request::CreateWindow {
            id: 7,
            parent: 0,
            x: 10,
            y: 10,
            width: 200,
            height: 100,
        },
    );

    assert_eq!(server.total_area(), 20_000);
    assert_eq!(server.mapped_total_area(), 0);
    assert!(snapshot.contains("forms_mapped: 0"));
    assert!(snapshot.contains("total_area: 0"));
    assert!(snapshot.contains("manifest_pressure_ppm: 0"));
}
