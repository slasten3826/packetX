use crate::compat::x11::X11Request;
use crate::server::{PacketAtom, PacketOrigin, ProcessKind};

pub fn birth_packet(packet_id: u64, tick: u64, request: &X11Request) -> PacketAtom {
    let origin = match request {
        X11Request::CreateWindow { .. }
        | X11Request::MapWindow { .. }
        | X11Request::ConfigureWindow { .. }
        | X11Request::UnmapWindow { .. } => PacketOrigin::X11Client,
    };

    let process_kind = ProcessKind::Scene;

    // Packet birth stays in chaos. Everything else only mutates or filters it.
    PacketAtom::new(packet_id, tick, origin, process_kind)
}
