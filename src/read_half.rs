use std::fmt;
use std::io::IoSliceMut;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use tokio::io::{AsyncRead, Interest, ReadBuf, Ready};

use crate::SocketAddr;
use crate::utils::Result;

///////////////////////////////////////////////////////////////////////////////

#[pin_project(project = ReadHalfProj)]
pub enum ReadHalf<'a> {
    Tcp(#[pin] tokio::net::tcp::ReadHalf<'a>),
    Unix(#[pin] tokio::net::unix::ReadHalf<'a>),
}

impl<'a> From<tokio::net::tcp::ReadHalf<'a>> for ReadHalf<'a> {
    fn from(x: tokio::net::tcp::ReadHalf<'a>) -> Self {
        Self::Tcp(x)
    }
}

impl<'a> From<tokio::net::unix::ReadHalf<'a>> for ReadHalf<'a> {
    fn from(x: tokio::net::unix::ReadHalf<'a>) -> Self {
        Self::Unix(x)
    }
}

impl ReadHalf<'_> {
    #[must_use]
    pub fn is_tcp(&self) -> bool {
        matches!(self, Self::Tcp(..))
    }

    #[must_use]
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::Unix(..))
    }
}

impl<'a> ReadHalf<'a> {
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

    pub async fn readable(&self) -> Result<()> {
        match self {
            Self::Tcp(x) => x.readable().await,
            Self::Unix(x) => x.readable().await,
        }
    }

    pub async fn ready(&self, interest: Interest) -> Result<Ready> {
        match self {
            Self::Tcp(x) => x.ready(interest).await,
            Self::Unix(x) => x.ready(interest).await,
        }
    }

    pub fn try_read(&self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_read(buf),
            Self::Unix(x) => x.try_read(buf),
        }
    }

    pub fn try_read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_read_vectored(bufs),
            Self::Unix(x) => x.try_read_vectored(bufs),
        }
    }
}

crate::macros::impl_async_read! {
    type: ReadHalf<'_>,
    proj: ReadHalfProj,
}

impl fmt::Debug for ReadHalf<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp(x) => x.fmt(f),
            Self::Unix(x) => x.fmt(f),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[pin_project(project = OwnedReadHalfProj)]
pub enum OwnedReadHalf {
    Tcp(#[pin] tokio::net::tcp::OwnedReadHalf),
    Unix(#[pin] tokio::net::unix::OwnedReadHalf),
}

impl From<tokio::net::tcp::OwnedReadHalf> for OwnedReadHalf {
    fn from(x: tokio::net::tcp::OwnedReadHalf) -> Self {
        Self::Tcp(x)
    }
}

impl From<tokio::net::unix::OwnedReadHalf> for OwnedReadHalf {
    fn from(x: tokio::net::unix::OwnedReadHalf) -> Self {
        Self::Unix(x)
    }
}

impl OwnedReadHalf {
    #[must_use]
    pub fn is_tcp(&self) -> bool {
        matches!(self, Self::Tcp(..))
    }

    #[must_use]
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::Unix(..))
    }
}

impl OwnedReadHalf {
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

    pub async fn readable(&self) -> Result<()> {
        match self {
            Self::Tcp(x) => x.readable().await,
            Self::Unix(x) => x.readable().await,
        }
    }

    pub async fn ready(&self, interest: Interest) -> Result<Ready> {
        match self {
            Self::Tcp(x) => x.ready(interest).await,
            Self::Unix(x) => x.ready(interest).await,
        }
    }

    pub fn try_read(&self, buf: &mut [u8]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_read(buf),
            Self::Unix(x) => x.try_read(buf),
        }
    }

    pub fn try_read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_read_vectored(bufs),
            Self::Unix(x) => x.try_read_vectored(bufs),
        }
    }
}

crate::macros::impl_async_read! {
    type: OwnedReadHalf,
    proj: OwnedReadHalfProj,
}

impl fmt::Debug for OwnedReadHalf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp(x) => x.fmt(f),
            Self::Unix(x) => x.fmt(f),
        }
    }
}
