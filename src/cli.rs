use std::path::PathBuf;

use x12_server::compat::x11::X11Bridge;
use x12_server::compat::x11::X11Request;
use x12_server::compat::x11::scenarios;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    ListScenarios,
    X11HandshakeOnce { socket_path: String },
    X11ClientOnce { socket_path: String },
    X11ClientSession { socket_path: String },
    X11ClientMulti { socket_path: String, client_count: usize },
    ScenarioRun {
        requests: Vec<X11Request>,
        ppm_dir: Option<PathBuf>,
    },
}

pub fn parse_args(args: &[String]) -> Result<Command, String> {
    let ppm_dir = flag_value(args, "--dump-ppm-dir").map(PathBuf::from);

    if has_flag(args, "--list-scenarios") {
        return Ok(Command::ListScenarios);
    }

    if let Some(socket_path) = flag_value(args, "--x11-handshake-once") {
        reject_ppm_with_wire(&ppm_dir, "--x11-handshake-once")?;
        return Ok(Command::X11HandshakeOnce {
            socket_path: socket_path.to_string(),
        });
    }

    if let Some(socket_path) = flag_value(args, "--x11-client-once") {
        reject_ppm_with_wire(&ppm_dir, "--x11-client-once")?;
        return Ok(Command::X11ClientOnce {
            socket_path: socket_path.to_string(),
        });
    }

    if let Some(socket_path) = flag_value(args, "--x11-client-session") {
        reject_ppm_with_wire(&ppm_dir, "--x11-client-session")?;
        return Ok(Command::X11ClientSession {
            socket_path: socket_path.to_string(),
        });
    }

    if let Some((socket_path, client_count)) = multi_client_args(args)? {
        reject_ppm_with_wire(&ppm_dir, "--x11-client-multi")?;
        return Ok(Command::X11ClientMulti {
            socket_path: socket_path.to_string(),
            client_count,
        });
    }

    let requests = if let Some(name) = flag_value(args, "--scenario") {
        scenarios::named(name).ok_or_else(|| {
            format!("unknown scenario: {name}\nuse --list-scenarios to inspect available scenarios")
        })?
    } else {
        X11Bridge::new().bootstrap_sequence()
    };

    Ok(Command::ScenarioRun { requests, ppm_dir })
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find_map(|window| (window[0] == flag).then_some(window[1].as_str()))
}

fn multi_client_args(args: &[String]) -> Result<Option<(&str, usize)>, String> {
    if let Some(window) = args.windows(3).find(|window| window[0] == "--x11-client-multi") {
        let client_count = window[2]
            .parse::<usize>()
            .map_err(|err| format!("invalid client count for --x11-client-multi: {err}"))?;
        Ok(Some((window[1].as_str(), client_count)))
    } else {
        Ok(None)
    }
}

fn reject_ppm_with_wire(ppm_dir: &Option<PathBuf>, wire_flag: &str) -> Result<(), String> {
    if let Some(dir) = ppm_dir {
        Err(format!(
            "--dump-ppm-dir {} cannot be combined with {}",
            dir.display(),
            wire_flag
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, parse_args};
    use std::path::PathBuf;

    #[test]
    fn parse_ppm_scenario_command() {
        let args = vec![
            "--scenario".to_string(),
            "partial-overlap".to_string(),
            "--dump-ppm-dir".to_string(),
            "/tmp/x12-frames".to_string(),
        ];

        let command = parse_args(&args).expect("scenario command should parse");
        match command {
            Command::ScenarioRun { ppm_dir, .. } => {
                assert_eq!(ppm_dir, Some(PathBuf::from("/tmp/x12-frames")));
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn reject_ppm_with_wire_mode() {
        let args = vec![
            "--x11-client-once".to_string(),
            "/tmp/x12.sock".to_string(),
            "--dump-ppm-dir".to_string(),
            "/tmp/x12-frames".to_string(),
        ];

        let err = parse_args(&args).expect_err("mixed wire and ppm args should fail");
        assert!(err.contains("--x11-client-once"));
    }
}
