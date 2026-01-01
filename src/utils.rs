use std::borrow::Cow;
use std::ffi::OsString;
use std::io::Error;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

#[cfg(target_os = "android")]
use std::os::android::net::SocketAddrExt;
#[cfg(target_os = "linux")]
use std::os::linux::net::SocketAddrExt;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[inline]
pub fn into2<A, B, C, D>((a, b): (A, B)) -> (C, D)
where
    A: Into<C>,
    B: Into<D>,
{
    (a.into(), b.into())
}

pub fn unix_addr_to_path<'a>(x: &'a std::os::unix::net::SocketAddr) -> Cow<'a, Path> {
    assert!(!x.is_unnamed());

    #[cfg(any(target_os = "linux", target_os = "android"))]
    if let Some(p) = x.as_abstract_name() {
        let mut path = Vec::with_capacity(p.len().strict_add(1));
        path.push(0);
        path.extend_from_slice(p);

        let path = OsString::from_vec(path);
        let path = PathBuf::from(path);
        return Cow::Owned(path);
    }

    let path = x
        .as_pathname()
        .expect("path should be Some because x is named");
    Cow::Borrowed(path)
}
