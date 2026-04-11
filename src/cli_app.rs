use std::fs;

use x12_server::compat::x11::scenarios;
use x12_server::compat::x11_wire::X11WireServer;
use x12_server::manifest::dump_front_buffer_ppm;
use x12_server::process_request;
use x12_server::run_sequence;
use x12_server::server::ServerState;

use crate::cli::Command;

pub fn run(command: Command) -> Result<(), String> {
    match command {
        Command::ListScenarios => {
            println!("available scenarios:");
            for name in scenarios::NAMES {
                println!("  {name}");
            }
            Ok(())
        }
        Command::X11HandshakeOnce { socket_path } => {
            let server = bind_wire_server(&socket_path)?;
            println!("x12 x11 wire listener waiting on {}", server.socket_path().display());
            let outcome = server
                .accept_setup_once()
                .map_err(|err| format!("x11 setup failed at transport layer: {err}"))?;
            println!("x11 setup outcome: {outcome:?}");
            Ok(())
        }
        Command::X11ClientOnce { socket_path } => {
            let server = bind_wire_server(&socket_path)?;
            let mut state = ServerState::new();
            println!(
                "x12 x11 wire client listener waiting on {}",
                server.socket_path().display()
            );
            let outcome = server
                .accept_client_once(&mut state)
                .map_err(|err| format!("x11 client failed at transport layer: {err}"))?;
            println!("x11 client outcome: {outcome:?}");
            Ok(())
        }
        Command::X11ClientSession { socket_path } => {
            let server = bind_wire_server(&socket_path)?;
            let mut state = ServerState::new();
            println!(
                "x12 x11 wire session listener waiting on {}",
                server.socket_path().display()
            );
            let outcome = server
                .accept_client_session(&mut state)
                .map_err(|err| format!("x11 session failed at transport layer: {err}"))?;
            println!("x11 session outcome: {outcome:?}");
            Ok(())
        }
        Command::X11ClientMulti {
            socket_path,
            client_count,
        } => {
            let server = bind_wire_server(&socket_path)?;
            let mut state = ServerState::new();
            println!(
                "x12 x11 multi-client listener waiting on {} for {} clients",
                server.socket_path().display(),
                client_count
            );
            let outcomes = server
                .accept_client_sessions(&mut state, client_count)
                .map_err(|err| format!("x11 multi-client failed at transport layer: {err}"))?;
            println!("x11 multi-client outcomes: {outcomes:#?}");
            Ok(())
        }
        Command::ScenarioRun { requests, ppm_dir } => {
            if let Some(dir) = ppm_dir {
                fs::create_dir_all(&dir).map_err(|err| {
                    format!("failed to create PPM output directory {}: {err}", dir.display())
                })?;

                let mut server = ServerState::new();
                for (index, request) in requests.iter().enumerate() {
                    let snapshot = process_request(&mut server, request);
                    println!("{snapshot}");

                    let path = dir.join(format!("frame-{index:03}.ppm"));
                    dump_front_buffer_ppm(&server, &path).map_err(|err| {
                        format!("failed to write PPM frame {}: {err}", path.display())
                    })?;
                }
                Ok(())
            } else {
                for snapshot in run_sequence(&requests) {
                    println!("{snapshot}");
                }
                Ok(())
            }
        }
    }
}

fn bind_wire_server(socket_path: &str) -> Result<X11WireServer, String> {
    X11WireServer::bind(socket_path)
        .map_err(|err| format!("failed to bind X11 wire socket at {socket_path}: {err}"))
}
