use std::fmt;
use std::io::Error;
use std::task::{Context, Poll};

use crate::utils::{Result, into2, unix_addr_to_path};
use crate::{SocketAddr, Stream, ToSocketAddrs};

///////////////////////////////////////////////////////////////////////////////

pub enum Listener {
    Tcp(tokio::net::TcpListener),
    Unix(tokio::net::UnixListener),
}

impl From<tokio::net::TcpListener> for Listener {
    #[inline]
    fn from(x: tokio::net::TcpListener) -> Self {
        Self::Tcp(x)
    }
}

impl From<tokio::net::UnixListener> for Listener {
    #[inline]
    fn from(x: tokio::net::UnixListener) -> Self {
        Self::Unix(x)
    }
}

impl Listener {
    #[must_use]
    pub fn is_tcp(&self) -> bool {
        matches!(self, Self::Tcp(..))
    }

    #[must_use]
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::Unix(..))
    }
}

impl Listener {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let addrs = addr.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            match Self::_bind(addr).await {
                Ok(x) => return Ok(x),
                Err(e) => last_err = Some(e),
            }
        }

        Err(last_err.unwrap())
    }

    async fn _bind(addr: SocketAddr) -> Result<Self> {
        match addr {
            SocketAddr::Tcp(x) => tokio::net::TcpListener::bind(x).await.map(Into::into),
            SocketAddr::Unix(x) => {
                assert!(!x.is_unnamed(), "cannot bind to an unnamed address");
                let x = x.into();
                tokio::net::UnixListener::bind(unix_addr_to_path(&x)).map(Into::into)
            }
        }
    }

    pub fn poll_accept(&self, cx: &mut Context<'_>) -> Poll<Result<(Stream, SocketAddr)>> {
        match self {
            Self::Tcp(x) => x.poll_accept(cx).map(|x| x.map(into2)),
            Self::Unix(x) => x.poll_accept(cx).map(|x| x.map(into2)),
        }
    }

    pub async fn accept(&self) -> Result<(Stream, SocketAddr)> {
        match self {
            Self::Tcp(x) => x.accept().await.map(into2),
            Self::Unix(x) => x.accept().await.map(into2),
        }
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        match self {
            Self::Tcp(x) => x.local_addr().map(Into::into),
            Self::Unix(x) => x.local_addr().map(Into::into),
        }
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        match self {
            Self::Tcp(_) => Ok(None),
            Self::Unix(x) => x.take_error(),
        }
    }
}

impl fmt::Debug for Listener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp(x) => x.fmt(f),
            Self::Unix(x) => x.fmt(f),
        }
    }
}
