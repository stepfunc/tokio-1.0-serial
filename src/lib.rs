pub use serialport::{DataBits, Error, ErrorKind, FlowControl, Parity, StopBits};

#[cfg(unix)]
use tokio::io::unix::AsyncFd;

use std::task::Context;
use tokio::io::ReadBuf;
use tokio::macros::support::{Pin, Poll};
use std::io::Write;
use std::io::Read;
use futures::ready;

pub struct Settings {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub flow_control: FlowControl,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

#[cfg(unix)]
pub struct AsyncSerial {
    inner: AsyncFd<serialport::TTYPort>,
}

#[cfg(windows)]
pub struct AsyncSerial;

#[cfg(unix)]
pub fn open(path: &str, settings: Settings) -> std::io::Result<AsyncSerial> {
    let tty = serialport::new(path, settings.baud_rate)
        .baud_rate(settings.baud_rate)
        .data_bits(settings.data_bits)
        .parity(settings.parity)
        .stop_bits(settings.stop_bits)
        .flow_control(settings.flow_control)
        .open_native()?;

    Ok(AsyncSerial { inner: AsyncFd::new(tty)? }) // TODO
}

/*
impl From<serialport::Error> for std::io::Error {
    fn from(err: Error) -> Self {
        match err.kind {
            serialport::ErrorKind::InvalidInput => std::io::ErrorKind::InvalidInput.into(),
            serialport::ErrorKind::Io(kind) => kind.into(),
            serialport::ErrorKind::NoDevice => std::io::ErrorKind::NotFound.into(),
            serialport::ErrorKind::Unknown => std::io::ErrorKind::Other,
        }
    }
}
*/ 

#[cfg(windows)]
pub fn open(_path: &str, _settings: Settings) -> Result<AsyncSerial, serialport::Error> {
    panic!("not supported on windows yet")
}

#[cfg(unix)]
impl tokio::io::AsyncRead for AsyncSerial {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<tokio::io::Result<()>> {
        let mut guard = ready!(self.inner.poll_read_ready(cx))?;
        match guard.try_io(|_| {
            let read = self.inner.get_ref().read(buf.initialize_unfilled())?;
            return Ok(buf.advance(read));
        }) {
            Ok(result) => return Poll::Ready(result),
            Err(_would_block) => return Poll::Pending,
        }
    }
}

#[cfg(unix)]
impl tokio::io::AsyncWrite for AsyncSerial {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<tokio::io::Result<usize>> {
        let mut guard = ready!(self.inner.poll_write_ready(cx))?;
        return match guard.try_io(|_| self.inner.get_ref().write(buf)) {
            Ok(x) => Poll::Ready(x),
            Err(_) => Poll::Pending,
        };
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<tokio::io::Result<()>> {
        let mut guard = ready!(self.inner.poll_write_ready(cx))?;
        let result = match guard.try_io(|_| self.inner.get_ref().flush()) {
            Ok(x) => Poll::Ready(x),
            Err(_) => Poll::Pending,
        };
        return result;
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<tokio::io::Result<()>> {
        return Poll::Ready(Ok(()));
    }
}
