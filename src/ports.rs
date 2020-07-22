use libc;
use std::fmt;
use std::io::{ErrorKind, Result};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

pub type PortList = Vec<Port>;
pub type PortNumber = u16;

#[derive(Clone, Copy, PartialEq)]
pub struct Port {
  pub number: PortNumber,
  pub status: PortStatus,
}

impl Port {
  #[allow(dead_code)]
  pub fn new(number: PortNumber) -> Port {
    Port {
      number,
      status: PortStatus::Unknown,
    }
  }

  pub fn from_str(input: &str) -> Option<Port> {
    let parsed = input.parse::<u16>();
    let number = parsed.unwrap() as PortNumber;
    Some(Port {
      number,
      status: PortStatus::Unknown,
    })
  }
}

impl std::fmt::Debug for Port {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Port")
      .field("number", &self.number)
      .field("status", &self.status)
      .finish()
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PortStatus {
  Open,
  Closed,
  Filtered,
  HostDown,
  Unknown,
}

impl fmt::Display for PortStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub fn probe_local_port(mut port: Port) -> Result<Port> {
  let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port.number);
  port.status = probe_port(&socket, Duration::from_millis(2569)).unwrap_or(PortStatus::HostDown);
  Ok(port)
}

pub fn probe_port(addr: &SocketAddr, timeout: Duration) -> Result<PortStatus> {
  match TcpStream::connect_timeout(&addr, timeout) {
    Ok(_) => Ok(PortStatus::Open),
    Err(e) => match e.kind() {
      ErrorKind::TimedOut => Ok(PortStatus::Filtered),
      ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset => Ok(PortStatus::Closed),
      _ => match e.raw_os_error() {
        Some(libc::ENETUNREACH) | Some(libc::EHOSTUNREACH) => Ok(PortStatus::HostDown),
        _ => Err(e),
      },
    },
  }
}
