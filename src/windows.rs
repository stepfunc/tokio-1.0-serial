use futures::task::Context;
use tokio::io::{Error, ReadBuf};
use tokio::macros::support::{Pin, Poll};

pub struct AsyncSerial;

pub fn open(_path: &str, _settings: super::Settings) -> std::io::Result<AsyncSerial> {
    unimplemented!()
}

impl tokio::io::AsyncRead for AsyncSerial {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &mut ReadBuf<'_>,
    ) -> Poll<tokio::io::Result<()>> {
        unimplemented!()
    }
}

impl tokio::io::AsyncWrite for AsyncSerial {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        unimplemented!()
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        unimplemented!()
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        unimplemented!()
    }
}
