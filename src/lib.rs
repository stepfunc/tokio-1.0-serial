pub use serialport::{DataBits, Error, ErrorKind, FlowControl, Parity, StopBits};

pub struct Settings {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub flow_control: FlowControl,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;


