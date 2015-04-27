use std::str::FromStr;
use std::path::PathBuf;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

use super::FromCommandLine;


impl FromCommandLine for PathBuf {
    fn from_argument(s: &str) -> Result<Self, String> {
        Ok(From::from(s))
    }
}

impl FromCommandLine for f32 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for f64 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}

// TODO(tailhook) implement various radices for integer values
impl FromCommandLine for isize {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for i8 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for i16 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for i32 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for i64 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for usize {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for u8 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for u16 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for u32 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for u64 {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for bool {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for String {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|_| unreachable!())
    }
}
impl FromCommandLine for Ipv4Addr {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for Ipv6Addr {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
impl FromCommandLine for SocketAddr {
    fn from_argument(s: &str) -> Result<Self, String> {
        FromStr::from_str(s).map_err(|e| format!("{:?}", e))
    }
}
