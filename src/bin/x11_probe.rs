use std::io::{Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;

#[derive(Debug, Clone)]
struct SetupSuccess {
    major: u16,
    minor: u16,
    vendor: String,
    xid_base: u32,
    xid_mask: u32,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!(
            "usage: x11_probe <socket-path> <create-window|map-window|unmap-window|configure-window|session-demo>"
        );
        std::process::exit(2);
    }

    let socket_path = &args[0];
    let command = &args[1];

    let mut stream = match UnixStream::connect(socket_path) {
        Ok(stream) => stream,
        Err(err) => {
            eprintln!("failed to connect to {socket_path}: {err}");
            std::process::exit(1);
        }
    };

    if let Err(err) = stream.write_all(&setup_request()) {
        eprintln!("failed to send setup request: {err}");
        std::process::exit(1);
    }

    let setup = match read_setup_response(&mut stream) {
        Ok(setup) => setup,
        Err(err) => {
            eprintln!("failed to read setup response: {err}");
            std::process::exit(1);
        }
    };

    match &setup {
        SetupResponse::Success(success) => println!(
            "setup: success version={}.{} vendor={} xid_base={:#010x} xid_mask={:#010x}",
            success.major, success.minor, success.vendor, success.xid_base, success.xid_mask
        ),
        SetupResponse::Failed { major, minor, reason } => {
            println!("setup: failed version={major}.{minor} reason={reason}");
            return;
        }
        SetupResponse::Unknown { status, major, minor } => {
            println!("setup: unknown status={status} version={major}.{minor}");
            return;
        }
    }

    let setup = match setup {
        SetupResponse::Success(success) => success,
        SetupResponse::Failed { .. } | SetupResponse::Unknown { .. } => unreachable!(),
    };

    let window_id = allocated_window_id(&setup);

    let requests = match command.as_str() {
        "create-window" => vec![create_window_request(window_id, 1, 10, 20, 640, 480)],
        "map-window" => vec![resource_request(8, window_id)],
        "unmap-window" => vec![resource_request(10, window_id)],
        "configure-window" => vec![configure_window_request(
            window_id,
            Some(30),
            None,
            Some(800),
            None,
        )],
        "session-demo" => vec![
            create_window_request(window_id, 1, 10, 20, 640, 480),
            resource_request(8, window_id),
            configure_window_request(window_id, Some(30), None, Some(800), None),
        ],
        other => {
            eprintln!("unknown command: {other}");
            std::process::exit(2);
        }
    };

    for request in requests {
        if let Err(err) = stream.write_all(&request) {
            eprintln!("failed to send request: {err}");
            std::process::exit(1);
        }
    }

    if let Err(err) = stream.shutdown(Shutdown::Write) {
        eprintln!("failed to close request stream: {err}");
        std::process::exit(1);
    }

    let mut maybe_error = [0u8; 32];
    match stream.read_exact(&mut maybe_error) {
        Ok(()) => {
            if maybe_error[0] == 0 {
                println!(
                    "xerror: code={} sequence={} major_opcode={}",
                    maybe_error[1],
                    u16::from_le_bytes([maybe_error[2], maybe_error[3]]),
                    maybe_error[10]
                );
            } else {
                println!("unexpected bytes after request: first_byte={}", maybe_error[0]);
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
            println!("request sequence sent; server closed connection without wire error");
        }
        Err(err) => {
            eprintln!("failed to read post-request state: {err}");
            std::process::exit(1);
        }
    }
}

fn setup_request() -> Vec<u8> {
    let mut out = Vec::with_capacity(12);
    out.push(b'l');
    out.push(0);
    out.extend_from_slice(&11u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&[0u8; 2]);
    out
}

enum SetupResponse {
    Success(SetupSuccess),
    Failed { major: u16, minor: u16, reason: String },
    Unknown { status: u8, major: u16, minor: u16 },
}

fn read_setup_response(stream: &mut UnixStream) -> std::io::Result<SetupResponse> {
    let mut prefix = [0u8; 8];
    stream.read_exact(&mut prefix)?;

    let status = prefix[0];
    let major = u16::from_le_bytes([prefix[2], prefix[3]]);
    let minor = u16::from_le_bytes([prefix[4], prefix[5]]);
    let length_units = u16::from_le_bytes([prefix[6], prefix[7]]) as usize;
    let trailing_len = length_units * 4;
    let mut trailing = vec![0u8; trailing_len];
    if trailing_len > 0 {
        stream.read_exact(&mut trailing)?;
    }

    match status {
        1 => {
            let xid_base = u32::from_le_bytes([trailing[4], trailing[5], trailing[6], trailing[7]]);
            let xid_mask =
                u32::from_le_bytes([trailing[8], trailing[9], trailing[10], trailing[11]]);
            let vendor_len = u16::from_le_bytes([trailing[16], trailing[17]]) as usize;
            let vendor_start = 32usize;
            let vendor_end = vendor_start + vendor_len;
            let vendor = String::from_utf8_lossy(&trailing[vendor_start..vendor_end]).to_string();
            Ok(SetupResponse::Success(SetupSuccess {
                major,
                minor,
                vendor,
                xid_base,
                xid_mask,
            }))
        }
        0 => {
            let reason_len = prefix[1] as usize;
            let reason = String::from_utf8_lossy(&trailing[..reason_len]).to_string();
            Ok(SetupResponse::Failed {
                major,
                minor,
                reason,
            })
        }
        other => Ok(SetupResponse::Unknown {
            status: other,
            major,
            minor,
        }),
    }
}

fn allocated_window_id(setup: &SetupSuccess) -> u32 {
    let low_bits = 0x004d & setup.xid_mask;
    setup.xid_base | low_bits
}

fn create_window_request(id: u32, parent: u32, x: i16, y: i16, width: u16, height: u16) -> Vec<u8> {
    let mut request = Vec::with_capacity(32);
    request.push(1);
    request.push(0);
    request.extend_from_slice(&8u16.to_le_bytes());
    request.extend_from_slice(&id.to_le_bytes());
    request.extend_from_slice(&parent.to_le_bytes());
    request.extend_from_slice(&x.to_le_bytes());
    request.extend_from_slice(&y.to_le_bytes());
    request.extend_from_slice(&width.to_le_bytes());
    request.extend_from_slice(&height.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&1u16.to_le_bytes());
    request.extend_from_slice(&0u32.to_le_bytes());
    request.extend_from_slice(&0u32.to_le_bytes());
    request
}

fn resource_request(opcode: u8, id: u32) -> Vec<u8> {
    let mut request = Vec::with_capacity(8);
    request.push(opcode);
    request.push(0);
    request.extend_from_slice(&2u16.to_le_bytes());
    request.extend_from_slice(&id.to_le_bytes());
    request
}

fn configure_window_request(
    id: u32,
    x: Option<i16>,
    y: Option<i16>,
    width: Option<u16>,
    height: Option<u16>,
) -> Vec<u8> {
    let mut mask = 0u16;
    let mut values = Vec::new();

    if let Some(x) = x {
        mask |= 1 << 0;
        values.extend_from_slice(&(x as i32 as u32).to_le_bytes());
    }
    if let Some(y) = y {
        mask |= 1 << 1;
        values.extend_from_slice(&(y as i32 as u32).to_le_bytes());
    }
    if let Some(width) = width {
        mask |= 1 << 2;
        values.extend_from_slice(&(width as u32).to_le_bytes());
    }
    if let Some(height) = height {
        mask |= 1 << 3;
        values.extend_from_slice(&(height as u32).to_le_bytes());
    }

    let length_units = 3 + (values.len() / 4) as u16;

    let mut request = Vec::with_capacity(length_units as usize * 4);
    request.push(12);
    request.push(0);
    request.extend_from_slice(&length_units.to_le_bytes());
    request.extend_from_slice(&id.to_le_bytes());
    request.extend_from_slice(&mask.to_le_bytes());
    request.extend_from_slice(&0u16.to_le_bytes());
    request.extend_from_slice(&values);
    request
}
