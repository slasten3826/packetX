use x12_server::compat::x11::X11Bridge;
use x12_server::compat::x11::scenarios;
use x12_server::compat::x11_wire::X11WireServer;
use x12_server::run_sequence;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

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

    for snapshot in run_sequence(&requests) {
        println!("{snapshot}");
    }
}
