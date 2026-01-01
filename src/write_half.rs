use std::fmt;
use std::io::IoSlice;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use tokio::io::{AsyncWrite, Interest, Ready};

use crate::SocketAddr;
use crate::utils::Result;

///////////////////////////////////////////////////////////////////////////////

#[pin_project(project = WriteHalfProj)]
pub enum WriteHalf<'a> {
    Tcp(#[pin] tokio::net::tcp::WriteHalf<'a>),
    Unix(#[pin] tokio::net::unix::WriteHalf<'a>),
}

impl<'a> From<tokio::net::tcp::WriteHalf<'a>> for WriteHalf<'a> {
    fn from(x: tokio::net::tcp::WriteHalf<'a>) -> Self {
        Self::Tcp(x)
    }
}

impl<'a> From<tokio::net::unix::WriteHalf<'a>> for WriteHalf<'a> {
    fn from(x: tokio::net::unix::WriteHalf<'a>) -> Self {
        Self::Unix(x)
    }
}

impl WriteHalf<'_> {
    #[must_use]
    pub fn is_tcp(&self) -> bool {
        matches!(self, Self::Tcp(..))
    }

    #[must_use]
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::Unix(..))
    }
}

impl<'a> WriteHalf<'a> {
    pub fn local_addr(&self) -> Result<SocketAddr> {
        match self {
            Self::Tcp(x) => x.local_addr().map(Into::into),
            Self::Unix(x) => x.local_addr().map(Into::into),
        }
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        match self {
            Self::Tcp(x) => x.peer_addr().map(Into::into),
            Self::Unix(x) => x.peer_addr().map(Into::into),
        }
    }

    pub async fn ready(&self, interest: Interest) -> Result<Ready> {
        match self {
            Self::Tcp(x) => x.ready(interest).await,
            Self::Unix(x) => x.ready(interest).await,
        }
    }

    pub fn try_write(&self, buf: &[u8]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_write(buf),
            Self::Unix(x) => x.try_write(buf),
        }
    }

    pub fn try_write_vectored(&self, bufs: &[IoSlice<'_>]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_write_vectored(bufs),
            Self::Unix(x) => x.try_write_vectored(bufs),
        }
    }

    pub async fn writable(&self) -> Result<()> {
        match self {
            Self::Tcp(x) => x.writable().await,
            Self::Unix(x) => x.writable().await,
        }
    }
}

crate::macros::impl_async_write! {
    type: WriteHalf<'_>,
    proj: WriteHalfProj,
}

impl fmt::Debug for WriteHalf<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp(x) => x.fmt(f),
            Self::Unix(x) => x.fmt(f),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[pin_project(project = OwnedWriteHalfProj)]
pub enum OwnedWriteHalf {
    Tcp(#[pin] tokio::net::tcp::OwnedWriteHalf),
    Unix(#[pin] tokio::net::unix::OwnedWriteHalf),
}

impl From<tokio::net::tcp::OwnedWriteHalf> for OwnedWriteHalf {
    fn from(x: tokio::net::tcp::OwnedWriteHalf) -> Self {
        Self::Tcp(x)
    }
}

impl From<tokio::net::unix::OwnedWriteHalf> for OwnedWriteHalf {
    fn from(x: tokio::net::unix::OwnedWriteHalf) -> Self {
        Self::Unix(x)
    }
}

impl OwnedWriteHalf {
    #[must_use]
    pub fn is_tcp(&self) -> bool {
        matches!(self, Self::Tcp(..))
    }

    #[must_use]
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::Unix(..))
    }
}

impl OwnedWriteHalf {
    pub fn forget(self) {
        match self {
            Self::Tcp(x) => x.forget(),
            Self::Unix(x) => x.forget(),
        }
    }

    pub fn local_addr(&self) -> Result<SocketAddr> {
        match self {
            Self::Tcp(x) => x.local_addr().map(Into::into),
            Self::Unix(x) => x.local_addr().map(Into::into),
        }
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        match self {
            Self::Tcp(x) => x.peer_addr().map(Into::into),
            Self::Unix(x) => x.peer_addr().map(Into::into),
        }
    }

    pub async fn ready(&self, interest: Interest) -> Result<Ready> {
        match self {
            Self::Tcp(x) => x.ready(interest).await,
            Self::Unix(x) => x.ready(interest).await,
        }
    }

    pub fn try_write(&self, buf: &[u8]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_write(buf),
            Self::Unix(x) => x.try_write(buf),
        }
    }

    pub fn try_write_vectored(&self, bufs: &[IoSlice<'_>]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_write_vectored(bufs),
            Self::Unix(x) => x.try_write_vectored(bufs),
        }
    }

    pub async fn writable(&self) -> Result<()> {
        match self {
            Self::Tcp(x) => x.writable().await,
            Self::Unix(x) => x.writable().await,
        }
    }
}

crate::macros::impl_async_write! {
    type: OwnedWriteHalf,
    proj: OwnedWriteHalfProj,
}

impl fmt::Debug for OwnedWriteHalf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp(x) => x.fmt(f),
            Self::Unix(x) => x.fmt(f),
        }
    }
}
