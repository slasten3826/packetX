#[derive(Debug, Clone)]
pub enum X11Request {
    CreateWindow {
        id: u32,
        parent: u32,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    },
    MapWindow {
        id: u32,
    },
    ConfigureWindow {
        id: u32,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    },
    UnmapWindow {
        id: u32,
    },
}

pub struct X11Bridge;

impl X11Bridge {
    pub fn new() -> Self {
        Self
    }

    pub fn bootstrap_sequence(&self) -> Vec<X11Request> {
        scenarios::bootstrap_sequence()
    }
}

pub mod scenarios {
    use super::X11Request;

    pub const NAMES: &[&str] = &[
        "bootstrap",
        "full-occlusion",
        "partial-overlap",
        "unmap-restore",
    ];

    pub fn named(name: &str) -> Option<Vec<X11Request>> {
        match name {
            "bootstrap" => Some(bootstrap_sequence()),
            "full-occlusion" => Some(full_occlusion_sequence()),
            "partial-overlap" => Some(partial_overlap_sequence()),
            "unmap-restore" => Some(unmap_restore_sequence()),
            _ => None,
        }
    }

    pub fn bootstrap_sequence() -> Vec<X11Request> {
        vec![
            X11Request::CreateWindow {
                id: 1,
                parent: 0,
                x: 10,
                y: 10,
                width: 640,
                height: 480,
            },
            X11Request::MapWindow { id: 1 },
            X11Request::CreateWindow {
                id: 2,
                parent: 0,
                x: 50,
                y: 40,
                width: 320,
                height: 240,
            },
            X11Request::MapWindow { id: 2 },
            X11Request::ConfigureWindow {
                id: 2,
                x: 60,
                y: 50,
                width: 320,
                height: 240,
            },
            X11Request::CreateWindow {
                id: 3,
                parent: 0,
                x: 500,
                y: 20,
                width: 120,
                height: 120,
            },
            X11Request::MapWindow { id: 3 },
            X11Request::UnmapWindow { id: 2 },
        ]
    }

    pub fn full_occlusion_sequence() -> Vec<X11Request> {
        vec![
            X11Request::CreateWindow {
                id: 1,
                parent: 0,
                x: 0,
                y: 0,
                width: 100,
                height: 100,
            },
            X11Request::MapWindow { id: 1 },
            X11Request::CreateWindow {
                id: 2,
                parent: 0,
                x: 0,
                y: 0,
                width: 100,
                height: 100,
            },
            X11Request::MapWindow { id: 2 },
        ]
    }

    pub fn partial_overlap_sequence() -> Vec<X11Request> {
        vec![
            X11Request::CreateWindow {
                id: 1,
                parent: 0,
                x: 0,
                y: 0,
                width: 100,
                height: 100,
            },
            X11Request::MapWindow { id: 1 },
            X11Request::CreateWindow {
                id: 2,
                parent: 0,
                x: 50,
                y: 0,
                width: 50,
                height: 100,
            },
            X11Request::MapWindow { id: 2 },
        ]
    }

    pub fn unmap_restore_sequence() -> Vec<X11Request> {
        let mut requests = full_occlusion_sequence();
        requests.push(X11Request::UnmapWindow { id: 2 });
        requests
    }
}
