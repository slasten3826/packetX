use std::collections::HashSet;
use std::fs;
use std::io::{self, Cursor, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::compat::x11::X11Request;
use crate::process_request_for_session;
use crate::server::ServerState;

const X_PROTOCOL_MAJOR: u16 = 11;
const X_PROTOCOL_MINOR: u16 = 0;
const SETUP_SUCCESS: u8 = 1;
const SETUP_FAILED: u8 = 0;
const IMAGE_ORDER_LSB_FIRST: u8 = 0;
const OPCODE_CREATE_WINDOW: u8 = 1;
const OPCODE_MAP_WINDOW: u8 = 8;
const OPCODE_UNMAP_WINDOW: u8 = 10;
const OPCODE_CONFIGURE_WINDOW: u8 = 12;
const SYNTHETIC_ROOT_WINDOW: u32 = 1;
const COPY_FROM_PARENT: u32 = 0;
const INPUT_OUTPUT: u16 = 1;
const CREATE_WINDOW_LENGTH_UNITS: u16 = 8;
const RESOURCE_REQ_LENGTH_UNITS: u16 = 2;
const CONFIGURE_WINDOW_BASE_UNITS: u16 = 3;
const CONFIGURE_MASK_ALLOWED: u16 = 0b1111;
const CLIENT_XID_BASE: u32 = 0x0020_0000;
const CLIENT_XID_MASK: u32 = 0x001f_ffff;
const CLIENT_XID_STRIDE: u32 = CLIENT_XID_MASK + 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X11ErrorCode {
    BadRequest = 1,
    BadValue = 2,
    BadWindow = 3,
    BadMatch = 8,
    BadLength = 16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct X11ProtocolError {
    pub code: X11ErrorCode,
    pub detail: String,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X11SetupRequest {
    pub byte_order: u8,
    pub major_version: u16,
    pub minor_version: u16,
    pub auth_proto_len: u16,
    pub auth_string_len: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum X11SetupOutcome {
    Success {
        vendor: &'static str,
        root_window: u32,
    },
    Failed {
        reason: String,
    },
}

pub struct X11WireServer {
    listener: UnixListener,
    socket_path: PathBuf,
    next_session_id: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct ClientSession {
    pub session_id: u64,
    pub next_sequence: u16,
    pub xid_base: u32,
    pub xid_mask: u32,
    pub setup_done: bool,
    owned_resources: HashSet<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum X11ClientOnceOutcome {
    SetupFailed {
        reason: String,
    },
    Snapshot {
        sequence_number: u16,
        snapshot: String,
    },
    ProtocolError {
        sequence_number: u16,
        error_code: X11ErrorCode,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum X11SessionOutcome {
    SetupFailed {
        reason: String,
    },
    Completed {
        processed_requests: usize,
        protocol_errors: usize,
        last_sequence: u16,
        last_snapshot: Option<String>,
    },
}

impl ClientSession {
    pub fn try_new(session_id: u64) -> Option<Self> {
        let xid_base = u32::try_from(session_id)
            .ok()
            .and_then(|slot| slot.checked_mul(CLIENT_XID_STRIDE))?;
        Some(Self {
            session_id,
            next_sequence: 1,
            xid_base,
            xid_mask: CLIENT_XID_MASK,
            setup_done: false,
            owned_resources: HashSet::new(),
        })
    }

    pub fn new(session_id: u64) -> Self {
        Self::try_new(session_id).expect("session xid space must exist")
    }

    fn take_sequence(&mut self) -> u16 {
        let seq = self.next_sequence;
        self.next_sequence = match self.next_sequence {
            u16::MAX => 1,
            current => current + 1,
        };
        seq
    }

    pub fn owns_xid(&self, xid: u32) -> bool {
        xid != 0 && (xid & !self.xid_mask) == self.xid_base
    }

    fn track_request(&mut self, request: &X11Request) {
        if let X11Request::CreateWindow { id, .. } = request {
            self.owned_resources.insert(*id);
        }
    }

    fn can_reference(&self, xid: u32) -> bool {
        xid == SYNTHETIC_ROOT_WINDOW || self.owned_resources.contains(&xid)
    }
}

impl X11WireServer {
    pub fn bind<P: AsRef<Path>>(socket_path: P) -> io::Result<Self> {
        let socket_path = socket_path.as_ref().to_path_buf();

        if socket_path.exists() {
            fs::remove_file(&socket_path)?;
        }

        let listener = UnixListener::bind(&socket_path)?;
        Ok(Self {
            listener,
            socket_path,
            next_session_id: AtomicU64::new(1),
        })
    }

    pub fn accept_setup_once(&self) -> io::Result<X11SetupOutcome> {
        let (mut stream, _) = self.listener.accept()?;
        handle_setup_stream(&mut stream)
    }

    pub fn accept_client_once(&self, server: &mut ServerState) -> io::Result<X11ClientOnceOutcome> {
        let (mut stream, _) = self.listener.accept()?;
        handle_client_once(&mut stream, server)
    }

    pub fn accept_client_session(&self, server: &mut ServerState) -> io::Result<X11SessionOutcome> {
        let (mut stream, _) = self.listener.accept()?;
        let mut session = self.allocate_session()?;
        handle_client_session(&mut stream, server, &mut session)
    }

    pub fn accept_client_sessions(
        &self,
        server: &mut ServerState,
        expected_clients: usize,
    ) -> io::Result<Vec<X11SessionOutcome>> {
        let mut outcomes = Vec::with_capacity(expected_clients);
        for _ in 0..expected_clients {
            let (mut stream, _) = self.listener.accept()?;
            let mut session = self.allocate_session()?;
            outcomes.push(handle_client_session(&mut stream, server, &mut session)?);
        }
        Ok(outcomes)
    }

    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    fn allocate_session_id(&self) -> u64 {
        self.next_session_id.fetch_add(1, Ordering::Relaxed)
    }

    fn allocate_session(&self) -> io::Result<ClientSession> {
        let session_id = self.allocate_session_id();
        ClientSession::try_new(session_id).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("x11 session xid space exhausted at session_id={session_id}"),
            )
        })
    }
}

impl Drop for X11WireServer {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.socket_path);
    }
}

pub fn handle_setup_stream(stream: &mut UnixStream) -> io::Result<X11SetupOutcome> {
    let request = read_setup_request(stream)?;
    discard_auth_payload(stream, request.auth_proto_len, request.auth_string_len)?;

    let outcome = validate_setup_request(request);
    let response = match &outcome {
        X11SetupOutcome::Success { .. } => build_setup_success(CLIENT_XID_BASE, CLIENT_XID_MASK),
        X11SetupOutcome::Failed { reason } => {
            build_setup_failure(request.major_version, request.minor_version, reason)
        }
    };

    stream.write_all(&response)?;
    stream.flush()?;
    Ok(outcome)
}

pub fn handle_client_once(
    stream: &mut UnixStream,
    server: &mut ServerState,
) -> io::Result<X11ClientOnceOutcome> {
    let request = read_setup_request(stream)?;
    discard_auth_payload(stream, request.auth_proto_len, request.auth_string_len)?;

    let setup_outcome = validate_setup_request(request);
    let setup_response = match &setup_outcome {
        X11SetupOutcome::Success { .. } => build_setup_success(CLIENT_XID_BASE, CLIENT_XID_MASK),
        X11SetupOutcome::Failed { reason } => {
            build_setup_failure(request.major_version, request.minor_version, reason)
        }
    };
    stream.write_all(&setup_response)?;
    stream.flush()?;

    match setup_outcome {
        X11SetupOutcome::Failed { reason } => Ok(X11ClientOnceOutcome::SetupFailed { reason }),
        X11SetupOutcome::Success { .. } => {
            let wire_request = read_wire_request(stream)?;
            match process_wire_request(server, 1, &wire_request) {
                Ok(snapshot) => Ok(X11ClientOnceOutcome::Snapshot {
                    sequence_number: 1,
                    snapshot,
                }),
                Err(error) => {
                    let error_code = decode_error_code(error[1]);
                    stream.write_all(&error)?;
                    stream.flush()?;
                    Ok(X11ClientOnceOutcome::ProtocolError {
                        sequence_number: 1,
                        error_code,
                    })
                }
            }
        }
    }
}

pub fn handle_setup_bytes(input: &[u8]) -> io::Result<(X11SetupOutcome, Vec<u8>)> {
    let mut cursor = Cursor::new(input);
    let request = read_setup_request(&mut cursor)?;
    discard_auth_payload(&mut cursor, request.auth_proto_len, request.auth_string_len)?;

    let outcome = validate_setup_request(request);
    let response = match &outcome {
        X11SetupOutcome::Success { .. } => build_setup_success(CLIENT_XID_BASE, CLIENT_XID_MASK),
        X11SetupOutcome::Failed { reason } => {
            build_setup_failure(request.major_version, request.minor_version, reason)
        }
    };

    Ok((outcome, response))
}

fn read_setup_request(reader: &mut impl Read) -> io::Result<X11SetupRequest> {
    let mut buf = [0u8; 12];
    reader.read_exact(&mut buf)?;

    Ok(X11SetupRequest {
        byte_order: buf[0],
        major_version: u16::from_le_bytes([buf[2], buf[3]]),
        minor_version: u16::from_le_bytes([buf[4], buf[5]]),
        auth_proto_len: u16::from_le_bytes([buf[6], buf[7]]),
        auth_string_len: u16::from_le_bytes([buf[8], buf[9]]),
    })
}

fn discard_auth_payload(
    reader: &mut impl Read,
    auth_proto_len: u16,
    auth_string_len: u16,
) -> io::Result<()> {
    let auth_proto_padded = padded_len(auth_proto_len as usize);
    let auth_string_padded = padded_len(auth_string_len as usize);
    let total = auth_proto_padded + auth_string_padded;

    if total == 0 {
        return Ok(());
    }

    let mut discard = vec![0u8; total];
    reader.read_exact(&mut discard)?;
    Ok(())
}

fn validate_setup_request(request: X11SetupRequest) -> X11SetupOutcome {
    if request.byte_order != b'l' {
        return X11SetupOutcome::Failed {
            reason: "x12 v0 only supports little-endian X11 clients".to_string(),
        };
    }

    if request.major_version != X_PROTOCOL_MAJOR || request.minor_version != X_PROTOCOL_MINOR {
        return X11SetupOutcome::Failed {
            reason: format!(
                "unsupported X11 protocol version {}.{}",
                request.major_version, request.minor_version
            ),
        };
    }

    if request.auth_proto_len != 0 || request.auth_string_len != 0 {
        return X11SetupOutcome::Failed {
            reason: "x12 v0 only supports empty X11 authorization".to_string(),
        };
    }

    X11SetupOutcome::Success {
        vendor: "packetX",
        root_window: SYNTHETIC_ROOT_WINDOW,
    }
}

fn build_setup_success(xid_base: u32, xid_mask: u32) -> Vec<u8> {
    let vendor = b"packetX";
    let vendor_padded_len = padded_len(vendor.len());

    let setup_payload_len = 32 + vendor_padded_len + 8 + 40;
    let length_units = (setup_payload_len / 4) as u16;

    let mut out = Vec::with_capacity(8 + setup_payload_len);

    out.push(SETUP_SUCCESS);
    out.push(0);
    out.extend_from_slice(&X_PROTOCOL_MAJOR.to_le_bytes());
    out.extend_from_slice(&X_PROTOCOL_MINOR.to_le_bytes());
    out.extend_from_slice(&length_units.to_le_bytes());

    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(&xid_base.to_le_bytes());
    out.extend_from_slice(&xid_mask.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(vendor.len() as u16).to_le_bytes());
    out.extend_from_slice(&u16::MAX.to_le_bytes());
    out.push(1);
    out.push(1);
    out.push(IMAGE_ORDER_LSB_FIRST);
    out.push(IMAGE_ORDER_LSB_FIRST);
    out.push(32);
    out.push(32);
    out.push(8);
    out.push(255);
    out.extend_from_slice(&[0u8; 4]);

    out.extend_from_slice(vendor);
    out.extend_from_slice(&vec![0u8; vendor_padded_len - vendor.len()]);

    out.push(24);
    out.push(32);
    out.push(32);
    out.extend_from_slice(&[0u8; 5]);

    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&0x00ff_ffffu32.to_le_bytes());
    out.extend_from_slice(&0x0000_0000u32.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&1920u16.to_le_bytes());
    out.extend_from_slice(&1080u16.to_le_bytes());
    out.extend_from_slice(&508u16.to_le_bytes());
    out.extend_from_slice(&285u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.push(0);
    out.push(0);
    out.push(24);
    out.push(0);

    debug_assert_eq!(out.len(), 8 + setup_payload_len);
    out
}

fn build_setup_failure(major_version: u16, minor_version: u16, reason: &str) -> Vec<u8> {
    let reason_bytes = reason.as_bytes();
    let reason_padded_len = padded_len(reason_bytes.len());
    let length_units = (reason_padded_len / 4) as u16;

    let mut out = Vec::with_capacity(8 + reason_padded_len);
    out.push(SETUP_FAILED);
    out.push(reason_bytes.len().min(u8::MAX as usize) as u8);
    out.extend_from_slice(&major_version.to_le_bytes());
    out.extend_from_slice(&minor_version.to_le_bytes());
    out.extend_from_slice(&length_units.to_le_bytes());
    out.extend_from_slice(reason_bytes);
    out.extend_from_slice(&vec![0u8; reason_padded_len - reason_bytes.len()]);
    out
}

fn padded_len(len: usize) -> usize {
    (len + 3) & !3
}

pub fn process_wire_request(
    server: &mut ServerState,
    sequence_number: u16,
    input: &[u8],
) -> Result<String, Vec<u8>> {
    let major_opcode = input.first().copied().unwrap_or(0);
    match parse_request_bytes(input) {
        Ok(request) => Ok(process_request_for_session(server, 0, &request)),
        Err(error) => Err(build_protocol_error(
            error.code,
            sequence_number,
            major_opcode,
            0,
        )),
    }
}

pub fn process_session_wire_request(
    server: &mut ServerState,
    session: &mut ClientSession,
    input: &[u8],
) -> Result<(u16, String), Vec<u8>> {
    let sequence_number = session.take_sequence();
    let major_opcode = input.first().copied().unwrap_or(0);

    match parse_request_bytes(input) {
        Ok(request) => {
            if let Err(error) = validate_session_request(session, &request) {
                return Err(build_protocol_error(
                    error.code,
                    sequence_number,
                    major_opcode,
                    0,
                ));
            }

            session.track_request(&request);
            let snapshot = process_request_for_session(server, session.session_id, &request);
            Ok((sequence_number, snapshot))
        }
        Err(error) => Err(build_protocol_error(
            error.code,
            sequence_number,
            major_opcode,
            0,
        )),
    }
}

pub fn handle_client_session(
    stream: &mut UnixStream,
    server: &mut ServerState,
    session: &mut ClientSession,
) -> io::Result<X11SessionOutcome> {
    if !server.register_client(session.session_id, session.xid_base, session.xid_mask) {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("duplicate session id {}", session.session_id),
        ));
    }

    let request = read_setup_request(stream)?;
    discard_auth_payload(stream, request.auth_proto_len, request.auth_string_len)?;

    let setup_outcome = validate_setup_request(request);
    let setup_response = match &setup_outcome {
        X11SetupOutcome::Success { .. } => build_setup_success(session.xid_base, session.xid_mask),
        X11SetupOutcome::Failed { reason } => {
            build_setup_failure(request.major_version, request.minor_version, reason)
        }
    };
    stream.write_all(&setup_response)?;
    stream.flush()?;

    match setup_outcome {
        X11SetupOutcome::Failed { reason } => {
            server.cleanup_session(session.session_id);
            Ok(X11SessionOutcome::SetupFailed { reason })
        }
        X11SetupOutcome::Success { .. } => {
            session.setup_done = true;
            server.mark_client_setup_done(session.session_id);
            let mut processed_requests = 0usize;
            let mut protocol_errors = 0usize;
            let mut last_sequence = 0u16;
            let mut last_snapshot = None;

            loop {
                let wire_request = match read_wire_request(stream) {
                    Ok(request) => request,
                    Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
                    Err(err) => return Err(err),
                };

                match process_session_wire_request(server, session, &wire_request) {
                    Ok((sequence, snapshot)) => {
                        processed_requests += 1;
                        last_sequence = sequence;
                        last_snapshot = Some(snapshot);
                    }
                    Err(error) => {
                        protocol_errors += 1;
                        last_sequence = u16::from_le_bytes([error[2], error[3]]);
                        stream.write_all(&error)?;
                        stream.flush()?;
                    }
                }
            }

            let outcome = X11SessionOutcome::Completed {
                processed_requests,
                protocol_errors,
                last_sequence,
                last_snapshot,
            };
            server.cleanup_session(session.session_id);
            Ok(outcome)
        }
    }
}

fn read_wire_request(stream: &mut UnixStream) -> io::Result<Vec<u8>> {
    let mut header = [0u8; 4];
    stream.read_exact(&mut header)?;
    let length_units = u16::from_le_bytes([header[2], header[3]]) as usize;
    let total_len = length_units
        .checked_mul(4)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "request length overflow"))?;

    if total_len < 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "wire request shorter than xReq header",
        ));
    }

    let mut request = header.to_vec();
    let trailing_len = total_len - 4;
    if trailing_len > 0 {
        let mut trailing = vec![0u8; trailing_len];
        stream.read_exact(&mut trailing)?;
        request.extend_from_slice(&trailing);
    }
    Ok(request)
}

pub fn parse_request_bytes(input: &[u8]) -> Result<X11Request, X11ProtocolError> {
    if input.len() < 4 {
        return Err(protocol_error(
            X11ErrorCode::BadLength,
            "xReq header is shorter than 4 bytes",
        ));
    }

    let req_type = input[0];
    let length_units = u16::from_le_bytes([input[2], input[3]]);
    let expected_len = length_units as usize * 4;

    if input.len() != expected_len {
        return Err(protocol_error(
            X11ErrorCode::BadLength,
            format!(
                "request buffer size {} does not match declared length {}",
                input.len(),
                expected_len
            ),
        ));
    }

    match req_type {
        OPCODE_CREATE_WINDOW => parse_create_window(input, length_units),
        OPCODE_MAP_WINDOW => parse_resource_window(input, length_units, true),
        OPCODE_UNMAP_WINDOW => parse_resource_window(input, length_units, false),
        OPCODE_CONFIGURE_WINDOW => parse_configure_window(input, length_units),
        _ => Err(protocol_error(
            X11ErrorCode::BadRequest,
            format!("unsupported X11 opcode {}", req_type),
        )),
    }
}

fn parse_create_window(input: &[u8], length_units: u16) -> Result<X11Request, X11ProtocolError> {
    if length_units != CREATE_WINDOW_LENGTH_UNITS {
        return Err(protocol_error(
            X11ErrorCode::BadLength,
            format!(
                "CreateWindow must have length {}, got {}",
                CREATE_WINDOW_LENGTH_UNITS, length_units
            ),
        ));
    }

    let id = le_u32(input, 4);
    let parent = le_u32(input, 8);
    let x = le_i16(input, 12);
    let y = le_i16(input, 14);
    let width = le_u16(input, 16);
    let height = le_u16(input, 18);
    let border_width = le_u16(input, 20);
    let class = le_u16(input, 22);
    let visual = le_u32(input, 24);
    let mask = le_u32(input, 28);

    if parent != SYNTHETIC_ROOT_WINDOW {
        return Err(protocol_error(
            X11ErrorCode::BadWindow,
            format!("CreateWindow parent {} is not synthetic root", parent),
        ));
    }

    if class != INPUT_OUTPUT {
        return Err(protocol_error(
            X11ErrorCode::BadMatch,
            format!("CreateWindow class {} is unsupported in v0", class),
        ));
    }

    if visual != COPY_FROM_PARENT {
        return Err(protocol_error(
            X11ErrorCode::BadMatch,
            format!("CreateWindow visual {} is unsupported in v0", visual),
        ));
    }

    if border_width != 0 || mask != 0 {
        return Err(protocol_error(
            X11ErrorCode::BadMatch,
            "CreateWindow only supports borderWidth=0 and mask=0 in v0",
        ));
    }

    if width == 0 || height == 0 {
        return Err(protocol_error(
            X11ErrorCode::BadValue,
            "CreateWindow width and height must be non-zero",
        ));
    }

    Ok(X11Request::CreateWindow {
        id,
        parent,
        x,
        y,
        width,
        height,
    })
}

fn parse_resource_window(
    input: &[u8],
    length_units: u16,
    map: bool,
) -> Result<X11Request, X11ProtocolError> {
    if length_units != RESOURCE_REQ_LENGTH_UNITS {
        return Err(protocol_error(
            X11ErrorCode::BadLength,
            format!(
                "resource request must have length {}, got {}",
                RESOURCE_REQ_LENGTH_UNITS, length_units
            ),
        ));
    }

    let id = le_u32(input, 4);
    if id == 0 {
        return Err(protocol_error(
            X11ErrorCode::BadWindow,
            "window id 0 is invalid for MapWindow/UnmapWindow",
        ));
    }

    Ok(if map {
        X11Request::MapWindow { id }
    } else {
        X11Request::UnmapWindow { id }
    })
}

fn parse_configure_window(
    input: &[u8],
    length_units: u16,
) -> Result<X11Request, X11ProtocolError> {
    if input.len() < 12 {
        return Err(protocol_error(
            X11ErrorCode::BadLength,
            "ConfigureWindow shorter than base header",
        ));
    }

    let id = le_u32(input, 4);
    if id == 0 {
        return Err(protocol_error(
            X11ErrorCode::BadWindow,
            "window id 0 is invalid for ConfigureWindow",
        ));
    }

    let mask = le_u16(input, 8);
    if mask & !CONFIGURE_MASK_ALLOWED != 0 {
        return Err(protocol_error(
            X11ErrorCode::BadMatch,
            format!("ConfigureWindow mask {:b} is unsupported in v0", mask),
        ));
    }

    let value_count = mask.count_ones() as usize;
    let expected_units = CONFIGURE_WINDOW_BASE_UNITS + value_count as u16;
    if length_units != expected_units {
        return Err(protocol_error(
            X11ErrorCode::BadLength,
            format!(
                "ConfigureWindow expected length {} from mask, got {}",
                expected_units, length_units
            ),
        ));
    }

    let mut cursor = 12usize;
    let mut next_value = || {
        let value = le_u32(input, cursor);
        cursor += 4;
        value
    };

    let x = if mask & 0b0001 != 0 {
        let value = next_value();
        let signed = i32::try_from(value).map_err(|_| {
            protocol_error(X11ErrorCode::BadValue, "ConfigureWindow x out of range")
        })?;
        Some(i16::try_from(signed).map_err(|_| {
            protocol_error(X11ErrorCode::BadValue, "ConfigureWindow x out of range")
        })?)
    } else {
        None
    };

    let y = if mask & 0b0010 != 0 {
        let value = next_value();
        let signed = i32::try_from(value).map_err(|_| {
            protocol_error(X11ErrorCode::BadValue, "ConfigureWindow y out of range")
        })?;
        Some(i16::try_from(signed).map_err(|_| {
            protocol_error(X11ErrorCode::BadValue, "ConfigureWindow y out of range")
        })?)
    } else {
        None
    };

    let width = if mask & 0b0100 != 0 {
        let value = next_value();
        Some(u16::try_from(value).map_err(|_| {
            protocol_error(X11ErrorCode::BadValue, "ConfigureWindow width out of range")
        })?)
    } else {
        None
    };

    let height = if mask & 0b1000 != 0 {
        let value = next_value();
        Some(u16::try_from(value).map_err(|_| {
            protocol_error(X11ErrorCode::BadValue, "ConfigureWindow height out of range")
        })?)
    } else {
        None
    };

    if width == Some(0) || height == Some(0) {
        return Err(protocol_error(
            X11ErrorCode::BadValue,
            "ConfigureWindow width/height must be non-zero when provided",
        ));
    }

    Ok(X11Request::ConfigureWindow {
        id,
        x,
        y,
        width,
        height,
    })
}

fn protocol_error(code: X11ErrorCode, detail: impl Into<String>) -> X11ProtocolError {
    X11ProtocolError {
        code,
        detail: detail.into(),
    }
}

fn validate_session_request(
    session: &ClientSession,
    request: &X11Request,
) -> Result<(), X11ProtocolError> {
    match request {
        X11Request::CreateWindow { id, parent, .. } => {
            if !session.owns_xid(*id) {
                return Err(protocol_error(
                    X11ErrorCode::BadValue,
                    format!("window id {} is outside the session xid space", id),
                ));
            }
            if session.owned_resources.contains(id) {
                return Err(protocol_error(
                    X11ErrorCode::BadValue,
                    format!("window id {} is already allocated in this session", id),
                ));
            }
            if !session.can_reference(*parent) {
                return Err(protocol_error(
                    X11ErrorCode::BadWindow,
                    format!("parent id {} is not owned by this session", parent),
                ));
            }
        }
        X11Request::MapWindow { id }
        | X11Request::UnmapWindow { id }
        | X11Request::ConfigureWindow { id, .. } => {
            if !session.can_reference(*id) {
                return Err(protocol_error(
                    X11ErrorCode::BadWindow,
                    format!("window id {} is not owned by this session", id),
                ));
            }
        }
    }

    Ok(())
}

fn build_protocol_error(
    code: X11ErrorCode,
    sequence_number: u16,
    major_opcode: u8,
    resource_id: u32,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(32);
    out.push(0);
    out.push(code as u8);
    out.extend_from_slice(&sequence_number.to_le_bytes());
    out.extend_from_slice(&resource_id.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.push(major_opcode);
    out.push(0);
    out.extend_from_slice(&[0u8; 20]);
    out
}

fn decode_error_code(raw: u8) -> X11ErrorCode {
    match raw {
        1 => X11ErrorCode::BadRequest,
        2 => X11ErrorCode::BadValue,
        3 => X11ErrorCode::BadWindow,
        8 => X11ErrorCode::BadMatch,
        16 => X11ErrorCode::BadLength,
        _ => X11ErrorCode::BadRequest,
    }
}

fn le_u16(input: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([input[offset], input[offset + 1]])
}

fn le_i16(input: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([input[offset], input[offset + 1]])
}

fn le_u32(input: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        input[offset],
        input[offset + 1],
        input[offset + 2],
        input[offset + 3],
    ])
}
