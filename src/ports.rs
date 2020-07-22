use libc;
use std::error::Error;
use std::fmt;
use std::io::{ErrorKind, Result};
use std::net::{SocketAddr, TcpStream};
use std::num::ParseIntError;
use std::time::Duration;

pub type PortList = Vec<Port>;
pub type PortNumber = i32;

pub struct Port {
  pub number: PortNumber,
  status: PortStatus,
}

impl Port {
  // This is a static method
  // Static methods don't need to be called by an instance
  // These methods are generally used as constructors
  pub fn origin() -> Port {
    Port {
      number: 0,
      status: PortStatus::Unknown,
    }
  }

  // Another static method, taking two arguments:
  pub fn new(number: PortNumber) -> Port {
    Port {
      number,
      status: PortStatus::Unknown,
    }
  }

  pub fn from_str(input: &str) -> Option<Port> {
    let parsed = input.parse::<i32>();
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
