use std::fmt;
use std::io::{Error, IoSlice, IoSliceMut};
use std::os::fd::{AsFd, AsRawFd};
use std::os::unix::prelude::{BorrowedFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use tokio::io::{AsyncRead, AsyncWrite, Interest, ReadBuf, Ready};

use crate::utils::{Result, into2, unix_addr_to_path};
use crate::{OwnedReadHalf, OwnedWriteHalf, ReadHalf, SocketAddr, ToSocketAddrs, WriteHalf};

///////////////////////////////////////////////////////////////////////////////

#[pin_project(project = StreamProj)]
pub enum Stream {
    Tcp(#[pin] tokio::net::TcpStream),
    Unix(#[pin] tokio::net::UnixStream),
}

impl From<tokio::net::TcpStream> for Stream {
    fn from(x: tokio::net::TcpStream) -> Self {
        Self::Tcp(x)
    }
}

impl From<tokio::net::UnixStream> for Stream {
    fn from(x: tokio::net::UnixStream) -> Self {
        Self::Unix(x)
    }
}

impl Stream {
    #[must_use]
    pub fn is_tcp(&self) -> bool {
        matches!(self, Self::Tcp(..))
    }

    #[must_use]
    pub fn is_unix(&self) -> bool {
        matches!(self, Self::Unix(..))
    }
}

impl Stream {
    pub async fn async_io<R>(&self, interest: Interest, f: impl FnMut() -> Result<R>) -> Result<R> {
        match self {
            Self::Tcp(x) => x.async_io(interest, f).await,
            Self::Unix(x) => x.async_io(interest, f).await,
        }
    }

    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let addrs = addr.to_socket_addrs()?;

        let mut last_err = None;
        for addr in addrs {
            match Self::_connect(addr).await {
                Ok(x) => return Ok(x),
                Err(e) => last_err = Some(e),
            }
        }

        Err(last_err.unwrap())
    }

    async fn _connect(addr: SocketAddr) -> Result<Self> {
        match addr {
            SocketAddr::Tcp(x) => tokio::net::TcpStream::connect(x).await.map(Into::into),
            SocketAddr::Unix(x) => {
                assert!(!x.is_unnamed(), "cannot connect to an unnamed address");
                let x = x.into();
                tokio::net::UnixStream::connect(unix_addr_to_path(&x))
                    .await
                    .map(Into::into)
            }
        }
    }

    pub fn into_split(self) -> (OwnedReadHalf, OwnedWriteHalf) {
        match self {
            Self::Tcp(x) => into2(x.into_split()),
            Self::Unix(x) => into2(x.into_split()),
        }
    }

    pub fn peer_addr(&self) -> Result<SocketAddr> {
        match self {
            Self::Tcp(x) => x.peer_addr().map(Into::into),
            Self::Unix(x) => x.peer_addr().map(Into::into),
        }
    }

    pub fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self {
            Self::Tcp(x) => x.poll_read_ready(cx),
            Self::Unix(x) => x.poll_write_ready(cx),
        }
    }

    pub fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self {
            Self::Tcp(x) => x.poll_write_ready(cx),
            Self::Unix(x) => x.poll_write_ready(cx),
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

    pub fn split<'a>(&'a mut self) -> (ReadHalf<'a>, WriteHalf<'a>) {
        match self {
            Self::Tcp(x) => into2(x.split()),
            Self::Unix(x) => into2(x.split()),
        }
    }

    pub fn take_error(&self) -> Result<Option<Error>> {
        match self {
            Self::Tcp(x) => x.take_error(),
            Self::Unix(x) => x.take_error(),
        }
    }

    pub fn try_io<R>(&self, interest: Interest, f: impl FnOnce() -> Result<R>) -> Result<R> {
        match self {
            Self::Tcp(x) => x.try_io(interest, f),
            Self::Unix(x) => x.try_io(interest, f),
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

    pub fn try_write(&self, buf: &[u8]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_write(buf),
            Self::Unix(x) => x.try_write(buf),
        }
    }

    pub fn try_write_vectored(&self, buf: &[IoSlice<'_>]) -> Result<usize> {
        match self {
            Self::Tcp(x) => x.try_write_vectored(buf),
            Self::Unix(x) => x.try_write_vectored(buf),
        }
    }

    pub async fn writable(&self) -> Result<()> {
        match self {
            Self::Tcp(x) => x.writable().await,
            Self::Unix(x) => x.writable().await,
        }
    }
}

impl AsFd for Stream {
    fn as_fd(&self) -> BorrowedFd<'_> {
        match self {
            Self::Tcp(x) => x.as_fd(),
            Self::Unix(x) => x.as_fd(),
        }
    }
}

impl AsRawFd for Stream {
    fn as_raw_fd(&self) -> RawFd {
        match self {
            Self::Tcp(x) => x.as_raw_fd(),
            Self::Unix(x) => x.as_raw_fd(),
        }
    }
}

crate::macros::impl_async_read_write! {
    type: Stream,
    proj: StreamProj,
}

impl fmt::Debug for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp(x) => x.fmt(f),
            Self::Unix(x) => x.fmt(f),
        }
    }
}
