use std::mem;

use crate::server::{FormAssembly, PacketAtom, PacketStatus, ServerState};

pub const DEFAULT_OUTPUT_WIDTH: u16 = 256;
pub const DEFAULT_OUTPUT_HEIGHT: u16 = 144;
const MANIFEST_BACKGROUND: u32 = 0xff10_1010;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestBuffer {
    width: u16,
    height: u16,
    pixels: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DamageRect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestState {
    front: ManifestBuffer,
    back: ManifestBuffer,
    damage: Vec<DamageRect>,
    dirty: bool,
}

impl Default for ManifestState {
    fn default() -> Self {
        Self::new(DEFAULT_OUTPUT_WIDTH, DEFAULT_OUTPUT_HEIGHT)
    }
}

impl ManifestBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let len = width as usize * height as usize;
        Self {
            width,
            height,
            pixels: vec![0; len],
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> Option<u32> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = y as usize * self.width as usize + x as usize;
        self.pixels.get(idx).copied()
    }

    pub fn fill_rect(&mut self, x: i16, y: i16, width: u16, height: u16, color: u32) {
        let Some((left, top, right, bottom)) = self.clip_rect(x, y, width, height) else {
            return;
        };

        let stride = self.width as usize;
        for row in top..bottom {
            let row_start = row as usize * stride;
            for col in left..right {
                self.pixels[row_start + col as usize] = color;
            }
        }
    }

    pub fn blit_form(&mut self, form: &FormAssembly, color: u32) {
        self.fill_rect(form.x, form.y, form.width, form.height, color);
    }

    fn clip_rect(
        &self,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Option<(u16, u16, u16, u16)> {
        let left = i32::from(x).max(0);
        let top = i32::from(y).max(0);
        let right = (i32::from(x) + i32::from(width)).min(i32::from(self.width));
        let bottom = (i32::from(y) + i32::from(height)).min(i32::from(self.height));

        if right <= left || bottom <= top {
            None
        } else {
            Some((left as u16, top as u16, right as u16, bottom as u16))
        }
    }
}

impl ManifestState {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            front: ManifestBuffer::new(width, height),
            back: ManifestBuffer::new(width, height),
            damage: Vec::new(),
            dirty: true,
        }
    }

    pub fn front(&self) -> &ManifestBuffer {
        &self.front
    }

    pub fn damage(&self) -> &[DamageRect] {
        &self.damage
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn render_forms(&mut self, forms: &[FormAssembly]) {
        if !self.dirty {
            self.damage.clear();
            return;
        }

        self.back.clear(MANIFEST_BACKGROUND);
        for form in forms.iter().filter(|form| contributes_to_manifest(form)) {
            self.back.blit_form(form, color_for_form(form.id));
        }
        self.damage = diff_damage(&self.front, &self.back);
        mem::swap(&mut self.front, &mut self.back);
        self.dirty = false;
    }
}

fn contributes_to_manifest(form: &FormAssembly) -> bool {
    form.mapped && form.visible
}

pub fn render_manifest_buffer(
    forms: &[FormAssembly],
    width: u16,
    height: u16,
) -> ManifestBuffer {
    let mut buffer = ManifestBuffer::new(width, height);
    buffer.clear(MANIFEST_BACKGROUND);

    for form in forms.iter().filter(|form| contributes_to_manifest(form)) {
        buffer.blit_form(form, color_for_form(form.id));
    }

    buffer
}

pub fn emit_snapshot(server: &mut ServerState, packet: &mut PacketAtom) -> String {
    let composited = server
        .forms
        .iter()
        .filter(|form| contributes_to_manifest(form))
        .count();
    server.manifest_state.render_forms(&server.forms);
    let damage = server.manifest_state.damage().to_vec();
    let output_width = server.manifest_state.front().width();
    let output_height = server.manifest_state.front().height();

    if matches!(packet.status, PacketStatus::Killed) {
        packet.log_ledger.push(format!(
            "manifest: synchronized current server state despite dead packet into {}x{} software buffer",
            output_width, output_height
        ));
    } else {
        packet.status = PacketStatus::Manifested;
        packet.log_ledger.push(format!(
            "manifest: composited {} visible forms into {}x{} software buffer",
            composited, output_width, output_height
        ));
    }
    packet.log_ledger.push(format!(
        "manifest: damage_rects={}",
        format_damage(&damage)
    ));
    packet
        .log_ledger
        .push("manifest: emitted text snapshot".to_string());

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
    out.push_str(&format!(
        "  output_surface: [{} x {}]\n",
        output_width, output_height
    ));
    out.push_str("  summary:\n");
    out.push_str(&format!("    forms_total: {}\n", server.forms.len()));
    out.push_str(&format!("    forms_mapped: {}\n", server.mapped_count()));
    out.push_str(&format!("    forms_visible: {}\n", server.visible_count()));
    out.push_str(&format!("    total_area: {}\n", total_area));
    out.push_str(&format!("    visible_area: {}\n", visible_area));
    out.push_str(&format!("    hidden_area: {}\n", hidden_area));
    out.push_str(&format!("    manifest_pressure_ppm: {}\n", pressure_ppm));
    out.push_str(&format!("    damage_rects: {}\n", damage.len()));
    out.push_str("  forms:\n");

    for form in &server.forms {
        let visible_ppm = if form.total_area == 0 {
            0
        } else {
            ((form.visible_area as u64 * 1_000_000) / form.total_area as u64) as u32
        };
        out.push_str(&format!(
            "    - id: {}\n      owner_session_id: {}\n      parent: {}\n      pos: [{}, {}]\n      size: [{}, {}]\n      mapped: {}\n      visible: {}\n      stacking: {}\n      total_area: {}\n      visible_area: {}\n      visible_ppm: {}\n      occluded_by: {}\n",
            form.id,
            form.owner_session_id,
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

fn diff_damage(front: &ManifestBuffer, back: &ManifestBuffer) -> Vec<DamageRect> {
    let mut min_x: Option<u16> = None;
    let mut min_y: Option<u16> = None;
    let mut max_x = 0u16;
    let mut max_y = 0u16;

    for y in 0..front.height() {
        for x in 0..front.width() {
            if front.get_pixel(x, y) != back.get_pixel(x, y) {
                min_x = Some(min_x.map_or(x, |current| current.min(x)));
                min_y = Some(min_y.map_or(y, |current| current.min(y)));
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    match (min_x, min_y) {
        (Some(x), Some(y)) => vec![DamageRect {
            x,
            y,
            width: max_x - x + 1,
            height: max_y - y + 1,
        }],
        _ => Vec::new(),
    }
}

fn format_damage(damage: &[DamageRect]) -> String {
    if damage.is_empty() {
        "[]".to_string()
    } else {
        let joined = damage
            .iter()
            .map(|rect| format!("[{},{} {}x{}]", rect.x, rect.y, rect.width, rect.height))
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{}]", joined)
    }
}

fn color_for_form(id: u32) -> u32 {
    let r = ((id.wrapping_mul(97) >> 0) & 0xff) as u32;
    let g = ((id.wrapping_mul(57) >> 3) & 0xff) as u32;
    let b = ((id.wrapping_mul(23) >> 5) & 0xff) as u32;
    0xff00_0000 | (r << 16) | (g << 8) | b
}
