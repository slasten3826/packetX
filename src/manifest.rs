use crate::server::{PacketAtom, PacketStatus, ServerState};

pub fn emit_snapshot(server: &ServerState, packet: &mut PacketAtom) -> String {
    if matches!(packet.status, PacketStatus::Killed) {
        packet
            .log_ledger
            .push("manifest: skipped because packet is dead".to_string());
    } else {
        packet.status = PacketStatus::Manifested;
        packet
            .log_ledger
            .push("manifest: emitted text snapshot".to_string());
    }

    let mut out = String::new();
    let total_area = server.mapped_total_area();
    let visible_area = server.visible_area();
    let hidden_area = total_area.saturating_sub(visible_area);
    let pressure_ppm = if total_area == 0 {
        0
    } else {
        ((visible_area as u64 * 1_000_000) / total_area as u64) as u32
    };

    out.push_str("snapshot:\n");
    out.push_str(&format!("  packet_id: {}\n", packet.id));
    out.push_str(&format!("  tick: {}\n", packet.birth_tick));
    out.push_str(&format!("  status: {:?}\n", packet.status));
    out.push_str(&format!("  origin: {:?}\n", packet.origin));
    out.push_str(&format!("  process_kind: {:?}\n", packet.process_kind));
    out.push_str("  summary:\n");
    out.push_str(&format!("    forms_total: {}\n", server.forms.len()));
    out.push_str(&format!("    forms_mapped: {}\n", server.mapped_count()));
    out.push_str(&format!("    forms_visible: {}\n", server.visible_count()));
    out.push_str(&format!("    total_area: {}\n", total_area));
    out.push_str(&format!("    visible_area: {}\n", visible_area));
    out.push_str(&format!("    hidden_area: {}\n", hidden_area));
    out.push_str(&format!("    manifest_pressure_ppm: {}\n", pressure_ppm));
    out.push_str("  forms:\n");

    for form in &server.forms {
        let visible_ppm = if form.total_area == 0 {
            0
        } else {
            ((form.visible_area as u64 * 1_000_000) / form.total_area as u64) as u32
        };
        out.push_str(&format!(
            "    - id: {}\n      parent: {}\n      pos: [{}, {}]\n      size: [{}, {}]\n      mapped: {}\n      visible: {}\n      stacking: {}\n      total_area: {}\n      visible_area: {}\n      visible_ppm: {}\n      occluded_by: {}\n",
            form.id,
            form.parent,
            form.x,
            form.y,
            form.width,
            form.height,
            form.mapped,
            form.visible,
            form.stacking_rank,
            form.total_area,
            form.visible_area,
            visible_ppm,
            form.occluded_by
                .map(|id| id.to_string())
                .unwrap_or_else(|| "null".to_string())
        ));
    }

    out.push_str("  log_ledger:\n");
    for entry in &packet.log_ledger {
        out.push_str(&format!("    - {entry}\n"));
    }

    out
}
