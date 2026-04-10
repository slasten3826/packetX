pub mod chaos;
pub mod compat;
pub mod crystall;
pub mod manifest;
pub mod server;
pub mod table;

use compat::x11::X11Request;
use server::ServerState;

pub fn run_sequence(requests: &[X11Request]) -> Vec<String> {
    let mut server = ServerState::new();
    let mut snapshots = Vec::with_capacity(requests.len());

    for request in requests {
        snapshots.push(process_request(&mut server, request));
    }

    snapshots
}

pub fn process_request(server: &mut ServerState, request: &X11Request) -> String {
    process_request_for_session(server, 0, request)
}

pub fn process_request_for_session(
    server: &mut ServerState,
    owner_session_id: u64,
    request: &X11Request,
) -> String {
    let tick = server.next_tick();
    let packet_id = server.next_packet_id();
    let mut packet = chaos::birth_packet(packet_id, tick, request);
    table::apply_request(server, &mut packet, owner_session_id, request);
    crystall::resolve_forms(server, &mut packet);
    manifest::emit_snapshot(server, &mut packet)
}
