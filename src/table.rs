use crate::compat::x11::X11Request;
use crate::server::{FormAssembly, PacketAtom, PacketStatus, ServerState};

pub fn apply_request(
    server: &mut ServerState,
    packet: &mut PacketAtom,
    owner_session_id: u64,
    request: &X11Request,
) {
    match request {
        X11Request::CreateWindow {
            id,
            parent,
            x,
            y,
            width,
            height,
        } => {
            let stacking_rank = server.forms.len();
            server.forms.push(FormAssembly {
                id: *id,
                owner_session_id,
                parent: *parent,
                x: *x,
                y: *y,
                width: *width,
                height: *height,
                mapped: false,
                stacking_rank,
                visible: false,
                occluded_by: None,
                total_area: (*width as u32) * (*height as u32),
                visible_area: 0,
            });
            packet
                .log_ledger
                .push(format!("table: create form claim id={id}"));
            server.mark_manifest_dirty();
            packet.status = PacketStatus::Tabled;
        }
        X11Request::MapWindow { id } => {
            if let Some(form) = server.form_mut(*id) {
                if owner_session_id != 0 && form.owner_session_id != owner_session_id {
                    packet.log_ledger.push(format!(
                        "table: ownership reject id={id} owner={} requester={owner_session_id}",
                        form.owner_session_id
                    ));
                    packet.status = PacketStatus::Killed;
                } else {
                    form.mapped = true;
                    packet.log_ledger.push(format!("table: map request id={id}"));
                    server.mark_manifest_dirty();
                    packet.status = PacketStatus::Tabled;
                }
            } else {
                packet
                    .log_ledger
                    .push(format!("table: null-task unknown form id={id}"));
                packet.status = PacketStatus::Killed;
            }
        }
        X11Request::ConfigureWindow {
            id,
            x,
            y,
            width,
            height,
        } => {
            if let Some(form) = server.form_mut(*id) {
                if owner_session_id != 0 && form.owner_session_id != owner_session_id {
                    packet.log_ledger.push(format!(
                        "table: ownership reject id={id} owner={} requester={owner_session_id}",
                        form.owner_session_id
                    ));
                    packet.status = PacketStatus::Killed;
                } else {
                    if let Some(x) = x {
                        form.x = *x;
                    }
                    if let Some(y) = y {
                        form.y = *y;
                    }
                    if let Some(width) = width {
                        form.width = *width;
                    }
                    if let Some(height) = height {
                        form.height = *height;
                    }
                    form.total_area = (form.width as u32) * (form.height as u32);
                    packet
                        .log_ledger
                        .push(format!("table: configure form id={id}"));
                    server.mark_manifest_dirty();
                    packet.status = PacketStatus::Tabled;
                }
            } else {
                packet
                    .log_ledger
                    .push(format!("table: null-task unknown form id={id}"));
                packet.status = PacketStatus::Killed;
            }
        }
        X11Request::UnmapWindow { id } => {
            if let Some(form) = server.form_mut(*id) {
                if owner_session_id != 0 && form.owner_session_id != owner_session_id {
                    packet.log_ledger.push(format!(
                        "table: ownership reject id={id} owner={} requester={owner_session_id}",
                        form.owner_session_id
                    ));
                    packet.status = PacketStatus::Killed;
                } else {
                    form.mapped = false;
                    form.visible = false;
                    form.occluded_by = None;
                    form.visible_area = 0;
                    packet.log_ledger.push(format!("table: unmap form id={id}"));
                    server.mark_manifest_dirty();
                    packet.status = PacketStatus::Tabled;
                }
            } else {
                packet
                    .log_ledger
                    .push(format!("table: null-task unknown form id={id}"));
                packet.status = PacketStatus::Killed;
            }
        }
    }
}
