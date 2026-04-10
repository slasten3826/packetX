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
}
