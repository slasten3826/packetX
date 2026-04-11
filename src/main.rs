use std::fs;
use std::path::PathBuf;

use x12_server::compat::x11::X11Bridge;
use x12_server::compat::x11::scenarios;
use x12_server::compat::x11_wire::X11WireServer;
use x12_server::manifest::dump_front_buffer_ppm;
use x12_server::process_request;
use x12_server::server::ServerState;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ppm_dir = args
        .windows(2)
        .find_map(|window| (window[0] == "--dump-ppm-dir").then_some(PathBuf::from(&window[1])));

    if args.iter().any(|arg| arg == "--list-scenarios") {
        println!("available scenarios:");
        for name in scenarios::NAMES {
            println!("  {name}");
        }
        return;
    }

    if let Some(socket_path) = args
        .windows(2)
        .find_map(|window| (window[0] == "--x11-handshake-once").then_some(window[1].as_str()))
    {
        let server = match X11WireServer::bind(socket_path) {
            Ok(server) => server,
            Err(err) => {
                eprintln!("failed to bind X11 wire socket at {socket_path}: {err}");
                std::process::exit(1);
            }
        };

        println!("x12 x11 wire listener waiting on {}", server.socket_path().display());
        match server.accept_setup_once() {
            Ok(outcome) => {
                println!("x11 setup outcome: {outcome:?}");
                return;
            }
            Err(err) => {
                eprintln!("x11 setup failed at transport layer: {err}");
                std::process::exit(1);
            }
        }
    }

    if let Some(socket_path) = args
        .windows(2)
        .find_map(|window| (window[0] == "--x11-client-once").then_some(window[1].as_str()))
    {
        let server = match X11WireServer::bind(socket_path) {
            Ok(server) => server,
            Err(err) => {
                eprintln!("failed to bind X11 wire socket at {socket_path}: {err}");
                std::process::exit(1);
            }
        };

        let mut state = x12_server::server::ServerState::new();
        println!("x12 x11 wire client listener waiting on {}", server.socket_path().display());
        match server.accept_client_once(&mut state) {
            Ok(outcome) => {
                println!("x11 client outcome: {outcome:?}");
                return;
            }
            Err(err) => {
                eprintln!("x11 client failed at transport layer: {err}");
                std::process::exit(1);
            }
        }
    }

    if let Some(socket_path) = args
        .windows(2)
        .find_map(|window| (window[0] == "--x11-client-session").then_some(window[1].as_str()))
    {
        let server = match X11WireServer::bind(socket_path) {
            Ok(server) => server,
            Err(err) => {
                eprintln!("failed to bind X11 wire socket at {socket_path}: {err}");
                std::process::exit(1);
            }
        };

        let mut state = x12_server::server::ServerState::new();
        println!("x12 x11 wire session listener waiting on {}", server.socket_path().display());
        match server.accept_client_session(&mut state) {
            Ok(outcome) => {
                println!("x11 session outcome: {outcome:?}");
                return;
            }
            Err(err) => {
                eprintln!("x11 session failed at transport layer: {err}");
                std::process::exit(1);
            }
        }
    }

    if let Some((socket_path, client_count)) = args.windows(3).find_map(|window| {
        if window[0] == "--x11-client-multi" {
            let count = window[2].parse::<usize>().ok()?;
            Some((window[1].as_str(), count))
        } else {
            None
        }
    }) {
        let server = match X11WireServer::bind(socket_path) {
            Ok(server) => server,
            Err(err) => {
                eprintln!("failed to bind X11 wire socket at {socket_path}: {err}");
                std::process::exit(1);
            }
        };

        let mut state = x12_server::server::ServerState::new();
        println!(
            "x12 x11 multi-client listener waiting on {} for {} clients",
            server.socket_path().display(),
            client_count
        );
        match server.accept_client_sessions(&mut state, client_count) {
            Ok(outcomes) => {
                println!("x11 multi-client outcomes: {outcomes:#?}");
                return;
            }
            Err(err) => {
                eprintln!("x11 multi-client failed at transport layer: {err}");
                std::process::exit(1);
            }
        }
    }

    let requests = if let Some(name) = args
        .windows(2)
        .find_map(|window| (window[0] == "--scenario").then_some(window[1].as_str()))
    {
        match scenarios::named(name) {
            Some(sequence) => sequence,
            None => {
                eprintln!("unknown scenario: {name}");
                eprintln!("use --list-scenarios to inspect available scenarios");
                std::process::exit(2);
            }
        }
    } else {
        let bridge = X11Bridge::new();
        bridge.bootstrap_sequence()
    };

    if let Some(dir) = ppm_dir {
        if let Err(err) = fs::create_dir_all(&dir) {
            eprintln!("failed to create PPM output directory {}: {err}", dir.display());
            std::process::exit(1);
        }

        let mut server = ServerState::new();
        for (index, request) in requests.iter().enumerate() {
            let snapshot = process_request(&mut server, request);
            println!("{snapshot}");

            let path = dir.join(format!("frame-{index:03}.ppm"));
            if let Err(err) = dump_front_buffer_ppm(&server, &path) {
                eprintln!("failed to write PPM frame {}: {err}", path.display());
                std::process::exit(1);
            }
        }
        return;
    }

    for snapshot in x12_server::run_sequence(&requests) {
        println!("{snapshot}");
    }
}
