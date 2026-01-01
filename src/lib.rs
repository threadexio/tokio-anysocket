//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc = include_str!("../README.md")]
#![cfg_attr(tokio_anysocket_nightly, feature(doc_cfg))]
#![forbid(unsafe_code)]

mod listener;
mod macros;
mod read_half;
mod socket_addr;
mod stream;
mod utils;
mod write_half;

pub use self::listener::Listener;
pub use self::read_half::{OwnedReadHalf, ReadHalf};
pub use self::socket_addr::{SocketAddr, ToSocketAddrs};
pub use self::stream::Stream;
pub use self::write_half::{OwnedWriteHalf, WriteHalf};
