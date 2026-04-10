use crate::server::{PacketAtom, PacketStatus, ServerState};

pub fn resolve_forms(server: &mut ServerState, packet: &mut PacketAtom) {
    if matches!(packet.status, PacketStatus::Killed) {
        packet
            .log_ledger
            .push("crystall: packet already dead before form stage".to_string());
        return;
    }

    let forms_snapshot = server.forms.clone();
    let mut any_visible = false;

    for (idx, form) in server.forms.iter_mut().enumerate() {
        if !form.mapped {
            form.visible = false;
            form.occluded_by = None;
            form.visible_area = 0;
            continue;
        }

        let mut killed_by = None;
        let mut hidden_area = 0u32;
        for upper in forms_snapshot.iter().skip(idx + 1) {
            if upper.mapped {
                let overlap = overlap_area(upper, form);
                if overlap == 0 {
                    continue;
                }

                hidden_area = hidden_area.saturating_add(overlap);

                if fully_covers(upper, form) {
                    killed_by = Some(upper.id);
                }
            }
        }

        form.occluded_by = killed_by;
        form.visible_area = form.total_area.saturating_sub(hidden_area.min(form.total_area));
        form.visible = form.visible_area > 0;
        any_visible |= form.visible;
    }

    if any_visible {
        let total_area = server.mapped_total_area();
        let visible_area = server.visible_area();
        let hidden_area = total_area.saturating_sub(visible_area);
        let pressure_ppm = if total_area == 0 {
            0
        } else {
            ((visible_area as u64 * 1_000_000) / total_area as u64) as u32
        };

        let visible_forms: Vec<String> = server
            .forms
            .iter()
            .filter(|form| form.visible)
            .map(|form| format!("{}#{}({}/{})", form.id, form.stacking_rank, form.visible_area, form.total_area))
            .collect();

        let occluded_forms: Vec<String> = server
            .forms
            .iter()
            .filter_map(|form| form.occluded_by.map(|upper| format!("{}<-{}", form.id, upper)))
            .collect();

        packet.log_ledger.push(format!(
            "crystall: visible_forms=[{}]",
            visible_forms.join(", ")
        ));
        if !occluded_forms.is_empty() {
            packet.log_ledger.push(format!(
                "crystall: occlusion_kills=[{}]",
                occluded_forms.join(", ")
            ));
        }
        packet.log_ledger.push(format!(
            "crystall: manifest_pressure visible_area={} hidden_area={} pressure_ppm={}",
            visible_area, hidden_area, pressure_ppm
        ));
        packet.status = PacketStatus::Crystallized;
    } else {
        packet
            .log_ledger
            .push("crystall: no visible forms, packet dies before manifest".to_string());
        packet.status = PacketStatus::Killed;
    }
}

fn fully_covers(upper: &crate::server::FormAssembly, lower: &crate::server::FormAssembly) -> bool {
    let upper_left = (upper.x as i32, upper.y as i32);
    let upper_right = (upper.x as i32 + upper.width as i32, upper.y as i32 + upper.height as i32);

    let lower_left = (lower.x as i32, lower.y as i32);
    let lower_right = (lower.x as i32 + lower.width as i32, lower.y as i32 + lower.height as i32);

    upper_left.0 <= lower_left.0
        && upper_left.1 <= lower_left.1
        && upper_right.0 >= lower_right.0
        && upper_right.1 >= lower_right.1
}

fn overlap_area(upper: &crate::server::FormAssembly, lower: &crate::server::FormAssembly) -> u32 {
    let left = (upper.x as i32).max(lower.x as i32);
    let top = (upper.y as i32).max(lower.y as i32);
    let right = (upper.x as i32 + upper.width as i32).min(lower.x as i32 + lower.width as i32);
    let bottom = (upper.y as i32 + upper.height as i32).min(lower.y as i32 + lower.height as i32);

    if right <= left || bottom <= top {
        0
    } else {
        ((right - left) * (bottom - top)) as u32
    }
}
