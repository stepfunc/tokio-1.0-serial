pub use serialport::{DataBits, Error, ErrorKind, FlowControl, Parity, StopBits};

pub struct Settings {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub flow_control: FlowControl,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

#[cfg(unix)]
pub struct SerialPort {
    tty: serialport::TTYPort,
}

#[cfg(windows)]
pub struct SerialPort;

#[cfg(unix)]
pub fn open(path: &str, settings: Settings) -> Result<SerialPort, serialport::Error> {
    let tty = serialport::new(path, settings.baud_rate)
        .baud_rate(settings.baud_rate)
        .data_bits(settings.data_bits)
        .parity(settings.parity)
        .stop_bits(settings.stop_bits)
        .flow_control(settings.flow_control)
        .open_native()?;

    Ok(SerialPort { tty })
}

#[cfg(windows)]
pub fn open(path: &Path, settings: Settings) -> Result<SerialPort, serialport::Error> {
    panic!("not supported on windows yet")
}
