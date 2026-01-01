macro_rules! impl_async_read {
    (
        type: $T:ty,
        proj: $proj:ident,
    ) => {
        impl AsyncRead for $T {
            fn poll_read(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                buf: &mut ReadBuf<'_>,
            ) -> Poll<Result<()>> {
                match self.project() {
                    $proj::Tcp(x) => x.poll_read(cx, buf),
                    $proj::Unix(x) => x.poll_read(cx, buf),
                }
            }
        }
    };
}

macro_rules! impl_async_write {
    (
        type: $T:ty,
        proj: $proj:ident,
    ) => {
        impl AsyncWrite for $T {
            fn poll_write(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                buf: &[u8],
            ) -> Poll<Result<usize>> {
                match self.project() {
                    $proj::Tcp(x) => x.poll_write(cx, buf),
                    $proj::Unix(x) => x.poll_write(cx, buf),
                }
            }

            fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
                match self.project() {
                    $proj::Tcp(x) => x.poll_flush(cx),
                    $proj::Unix(x) => x.poll_flush(cx),
                }
            }

            fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
                match self.project() {
                    $proj::Tcp(x) => x.poll_shutdown(cx),
                    $proj::Unix(x) => x.poll_shutdown(cx),
                }
            }

            fn poll_write_vectored(
                self: Pin<&mut Self>,
                cx: &mut Context<'_>,
                bufs: &[IoSlice<'_>],
            ) -> Poll<Result<usize>> {
                match self.project() {
                    $proj::Tcp(x) => x.poll_write_vectored(cx, bufs),
                    $proj::Unix(x) => x.poll_write_vectored(cx, bufs),
                }
            }

            fn is_write_vectored(&self) -> bool {
                match self {
                    Self::Tcp(x) => x.is_write_vectored(),
                    Self::Unix(x) => x.is_write_vectored(),
                }
            }
        }
    };
}

macro_rules! impl_async_read_write {
    ($($tt:tt)*) => {
        $crate::macros::impl_async_read! { $($tt)* }
        $crate::macros::impl_async_write! { $($tt)* }
    }
}

pub(crate) use impl_async_read;
pub(crate) use impl_async_read_write;
pub(crate) use impl_async_write;
