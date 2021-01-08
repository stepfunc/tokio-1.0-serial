pub use serialport::{DataBits, Error, ErrorKind, FlowControl, Parity, StopBits};

pub struct Settings {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub flow_control: FlowControl,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

impl Settings {
    #[cfg(unix)]
    pub(crate) fn build(&self, builder: serialport::SerialPortBuilder) -> serialport::SerialPortBuilder {
        builder
            .data_bits(self.data_bits)
            .flow_control(self.flow_control)
            .stop_bits(self.stop_bits)
            .parity(self.parity)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            stop_bits: StopBits::One,
            parity: Parity::None,
        }
    }
}

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;
