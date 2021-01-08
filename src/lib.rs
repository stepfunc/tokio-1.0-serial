pub use serialport::{DataBits, StopBits, Parity, FlowControl, Error, ErrorKind};
use std::path::Path;

pub struct Settings {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub flow_control: FlowControl,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

#[cfg(unix)]
pub struct SerialPort {
    inner: serialport::TTYPort,
}

#[cfg(windows)]
pub struct SerialPort;

#[cfg(unix)]
pub fn open(path: impl Into<Path>, settings: Settings) -> Result<SerialPort, serialport::Error> {
    // todo
}

#[cfg(windows)]
pub fn open(path: &Path, settings: Settings) -> Result<SerialPort, serialport::Error> {
    panic!("not supported on windows yet")
}

