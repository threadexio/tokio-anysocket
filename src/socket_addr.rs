use std::ffi::OsStr;
use std::fmt;
use std::io::Error;
use std::iter;
use std::os::unix::ffi::OsStrExt;
use std::str::FromStr;
use std::vec;

#[cfg(target_os = "android")]
use std::os::android::net::SocketAddrExt;
#[cfg(target_os = "linux")]
use std::os::linux::net::SocketAddrExt;

use crate::utils::Result;

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum SocketAddr {
    Tcp(std::net::SocketAddr),
    Unix(tokio::net::unix::SocketAddr),
}

impl From<std::net::SocketAddr> for SocketAddr {
    fn from(x: std::net::SocketAddr) -> Self {
        Self::Tcp(x)
    }
}

impl From<tokio::net::unix::SocketAddr> for SocketAddr {
    fn from(x: tokio::net::unix::SocketAddr) -> Self {
        Self::Unix(x)
    }
}

impl SocketAddr {
    #[must_use]
    pub fn is_tcp(&self) -> bool {
        matches!(self, Self::Tcp(..))
    }

    #[must_use]
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::Unix(..))
    }
}

impl fmt::Debug for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp(x) => write!(f, "tcp://{x}"),
            Self::Unix(x) => {
                let x = std::os::unix::net::SocketAddr::from(x.clone());

                if x.is_unnamed() {
                    return f.write_str("(unnamed unix socket)");
                }

                f.write_str("unix://")?;

                #[cfg(any(target_os = "linux", target_os = "android"))]
                if let Some(p) = x.as_abstract_name() {
                    let name = OsStr::from_bytes(p);
                    return write!(f, "@{}", name.display());
                }

                let path = x
                    .as_pathname()
                    .expect("path should be Some because x is named");

                write!(f, "{}", path.display())
            }
        }
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for SocketAddr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(x) = s.strip_prefix("tcp://") {
            x.parse().map(SocketAddr::Tcp).map_err(Error::other)
        } else if let Some(x) = s.strip_prefix("unix://") {
            fn parse_unix_addr(x: &str) -> Result<tokio::net::unix::SocketAddr> {
                #[cfg(any(target_os = "linux", target_os = "android"))]
                if let Some(x) = x.strip_prefix('@') {
                    return std::os::unix::net::SocketAddr::from_abstract_name(x.as_bytes())
                        .map(Into::into)
                        .map_err(Error::other);
                }

                std::os::unix::net::SocketAddr::from_pathname(x)
                    .map(Into::into)
                    .map_err(Error::other)
            }

            parse_unix_addr(x).map(SocketAddr::Unix)
        } else {
            Err(Error::other("invalid scheme"))
        }
    }
}

impl TryFrom<&str> for SocketAddr {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<String> for SocketAddr {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::de::{Error, Visitor};
    use serde::{Deserialize, Deserializer};

    impl<'de> Deserialize<'de> for SocketAddr {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct SocketAddrVisitor;

            impl<'de> Visitor<'de> for SocketAddrVisitor {
                type Value = SocketAddr;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str("a socket address")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    v.parse().map_err(Error::custom)
                }

                fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    v.parse().map_err(Error::custom)
                }
            }

            deserializer.deserialize_str(SocketAddrVisitor)
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub trait ToSocketAddrs {
    type Iter: Iterator<Item = SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter>;
}

impl<T> ToSocketAddrs for &T
where
    T: ToSocketAddrs + ?Sized,
{
    type Iter = T::Iter;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        T::to_socket_addrs(self)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl ToSocketAddrs for SocketAddr {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        Ok(iter::once(self.clone()))
    }
}

impl ToSocketAddrs for str {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        self.parse().map(iter::once)
    }
}

impl ToSocketAddrs for String {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        self.parse().map(iter::once)
    }
}

impl<T> ToSocketAddrs for &[T]
where
    T: ToSocketAddrs,
{
    type Iter = vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        let mut addrs = Vec::new();

        for item in self.iter() {
            addrs.extend(item.to_socket_addrs()?);
        }

        Ok(addrs.into_iter())
    }
}

impl ToSocketAddrs for std::net::SocketAddr {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        Ok(iter::once(SocketAddr::Tcp(*self)))
    }
}

impl ToSocketAddrs for std::net::SocketAddrV4 {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        Ok(iter::once(SocketAddr::Tcp(std::net::SocketAddr::V4(*self))))
    }
}

impl ToSocketAddrs for std::net::SocketAddrV6 {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        Ok(iter::once(SocketAddr::Tcp(std::net::SocketAddr::V6(*self))))
    }
}

impl ToSocketAddrs for (std::net::IpAddr, u16) {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        Ok(iter::once(SocketAddr::Tcp(std::net::SocketAddr::new(
            self.0, self.1,
        ))))
    }
}

impl ToSocketAddrs for (std::net::Ipv4Addr, u16) {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        Ok(iter::once(SocketAddr::Tcp(std::net::SocketAddr::V4(
            std::net::SocketAddrV4::new(self.0, self.1),
        ))))
    }
}

impl ToSocketAddrs for (std::net::Ipv6Addr, u16) {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        Ok(iter::once(SocketAddr::Tcp(std::net::SocketAddr::V6(
            std::net::SocketAddrV6::new(self.0, self.1, 0, 0),
        ))))
    }
}

impl ToSocketAddrs for std::path::Path {
    type Iter = iter::Once<SocketAddr>;

    fn to_socket_addrs(&self) -> Result<Self::Iter> {
        #[cfg(any(target_os = "linux", target_os = "android"))]
        if let Ok(path) = self.strip_prefix("@") {
            return std::os::unix::net::SocketAddr::from_abstract_name(path.as_os_str().as_bytes())
                .map(Into::into)
                .map(SocketAddr::Unix)
                .map(iter::once)
                .map_err(Error::other);
        }

        std::os::unix::net::SocketAddr::from_pathname(self)
            .map(Into::into)
            .map(SocketAddr::Unix)
            .map(iter::once)
            .map_err(Error::other)
    }
}
