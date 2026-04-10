use x12_server::compat::x11::X11Bridge;
use x12_server::compat::x11::scenarios;
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
