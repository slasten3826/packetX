use std::collections::HashMap;

use crate::manifest::{ManifestState, DEFAULT_OUTPUT_HEIGHT, DEFAULT_OUTPUT_WIDTH};

#[derive(Debug, Clone, Copy)]
pub enum PacketOrigin {
    X11Client,
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessKind {
    Scene,
}

#[derive(Debug, Clone, Copy)]
pub enum PacketStatus {
    Born,
    Tabled,
    Crystallized,
    Manifested,
    Killed,
}

#[derive(Debug, Clone)]
pub struct PacketAtom {
    pub id: u64,
    pub birth_tick: u64,
    pub origin: PacketOrigin,
    pub process_kind: ProcessKind,
    pub status: PacketStatus,
    pub log_ledger: Vec<String>,
}

impl PacketAtom {
    pub fn new(id: u64, birth_tick: u64, origin: PacketOrigin, process_kind: ProcessKind) -> Self {
        Self {
            id,
            birth_tick,
            origin,
            process_kind,
            status: PacketStatus::Born,
            log_ledger: vec!["chaos: packet born".to_string()],
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormAssembly {
    pub id: u32,
    pub owner_session_id: u64,
    pub parent: u32,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub mapped: bool,
    pub stacking_rank: usize,
    pub visible: bool,
    pub occluded_by: Option<u32>,
    pub total_area: u32,
    pub visible_area: u32,
}

#[derive(Debug, Clone)]
pub struct ServerClient {
    pub session_id: u64,
    pub xid_base: u32,
    pub xid_mask: u32,
    pub setup_done: bool,
}

#[derive(Debug, Default)]
pub struct ServerState {
    next_tick: u64,
    next_packet: u64,
    pub forms: Vec<FormAssembly>,
    pub clients: HashMap<u64, ServerClient>,
    pub manifest_state: ManifestState,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            next_tick: 0,
            next_packet: 1,
            forms: Vec::new(),
            clients: HashMap::new(),
            manifest_state: ManifestState::new(DEFAULT_OUTPUT_WIDTH, DEFAULT_OUTPUT_HEIGHT),
        }
    }

    pub fn next_tick(&mut self) -> u64 {
        self.next_tick += 1;
        self.next_tick
    }

    pub fn next_packet_id(&mut self) -> u64 {
        let id = self.next_packet;
        self.next_packet += 1;
        id
    }

    pub fn form_mut(&mut self, id: u32) -> Option<&mut FormAssembly> {
        self.forms.iter_mut().find(|form| form.id == id)
    }

    pub fn form(&self, id: u32) -> Option<&FormAssembly> {
        self.forms.iter().find(|form| form.id == id)
    }

    pub fn forms_for_session(&self, owner_session_id: u64) -> impl Iterator<Item = &FormAssembly> {
        self.forms
            .iter()
            .filter(move |form| form.owner_session_id == owner_session_id)
    }

    pub fn register_client(&mut self, session_id: u64, xid_base: u32, xid_mask: u32) -> bool {
        self.clients
            .insert(
                session_id,
                ServerClient {
                    session_id,
                    xid_base,
                    xid_mask,
                    setup_done: false,
                },
            )
            .is_none()
    }

    pub fn mark_client_setup_done(&mut self, session_id: u64) -> bool {
        if let Some(client) = self.clients.get_mut(&session_id) {
            client.setup_done = true;
            true
        } else {
            false
        }
    }

    pub fn client(&self, session_id: u64) -> Option<&ServerClient> {
        self.clients.get(&session_id)
    }

    pub fn mark_manifest_dirty(&mut self) {
        self.manifest_state.mark_dirty();
    }

    pub fn cleanup_session(&mut self, session_id: u64) {
        self.clients.remove(&session_id);
        self.forms.retain(|form| form.owner_session_id != session_id);
        for (stacking_rank, form) in self.forms.iter_mut().enumerate() {
            form.stacking_rank = stacking_rank;
        }
        self.mark_manifest_dirty();
    }

    pub fn total_area(&self) -> u32 {
        self.forms.iter().map(|form| form.total_area).sum()
    }

    pub fn mapped_total_area(&self) -> u32 {
        self.forms
            .iter()
            .filter(|form| form.mapped)
            .map(|form| form.total_area)
            .sum()
    }

    pub fn visible_area(&self) -> u32 {
        self.forms.iter().map(|form| form.visible_area).sum()
    }

    pub fn mapped_count(&self) -> usize {
        self.forms.iter().filter(|form| form.mapped).count()
    }

    pub fn visible_count(&self) -> usize {
        self.forms.iter().filter(|form| form.visible).count()
    }
}
