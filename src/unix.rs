use futures::ready;
use std::io::Read;
use std::io::Write;
use std::task::Context;
use tokio::io::unix::AsyncFd;
use tokio::io::ReadBuf;
use tokio::macros::support::{Pin, Poll};

pub struct AsyncSerial {
    inner: AsyncFd<serialport::TTYPort>,
}

pub fn open(path: &str, settings: super::Settings) -> std::io::Result<AsyncSerial> {
    let tty = settings
        .build(serialport::new(path, settings.baud_rate))
        .open_native()?;

    Ok(AsyncSerial {
        inner: AsyncFd::new(tty)?,
    })
}

impl tokio::io::AsyncRead for AsyncSerial {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<tokio::io::Result<()>> {
        let mut guard = ready!(self.inner.poll_read_ready_mut(cx))?;
        match guard.try_io(|inner| {
            let read = inner.get_mut().read(buf.initialize_unfilled())?;
            buf.advance(read);
            Ok(())
        }) {
            Ok(result) => Poll::Ready(result),
            Err(_would_block) => Poll::Pending,
        }
    }
}

impl tokio::io::AsyncWrite for AsyncSerial {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<tokio::io::Result<usize>> {
        let mut guard = ready!(self.inner.poll_write_ready_mut(cx))?;
        match guard.try_io(|io| io.get_mut().write(buf)) {
            Ok(x) => Poll::Ready(x),
            Err(_) => Poll::Pending,
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<tokio::io::Result<()>> {
        let mut guard = ready!(self.inner.poll_write_ready_mut(cx))?;
        match guard.try_io(|io| io.get_mut().flush()) {
            Ok(x) => Poll::Ready(x),
            Err(_) => Poll::Pending,
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<tokio::io::Result<()>> {
        // the underlying TTYPort is close-on-drop
        Poll::Ready(Ok(()))
    }
}
