mod chaos;
mod compat;
mod crystall;
mod manifest;
mod server;
mod table;

use compat::x11::X11Bridge;
use server::ServerState;

fn main() {
    let mut server = ServerState::new();
    let bridge = X11Bridge::new();

    for request in bridge.bootstrap_sequence() {
        let tick = server.next_tick();
        let packet_id = server.next_packet_id();
        let mut packet = chaos::birth_packet(packet_id, tick, &request);
        table::apply_request(&mut server, &mut packet, &request);
        crystall::resolve_forms(&mut server, &mut packet);
        let snapshot = manifest::emit_snapshot(&server, &mut packet);
        println!("{snapshot}");
    }
}
