#[cfg(windows)]
pub struct AsyncSerial;


#[cfg(windows)]
pub fn open(_path: &str, _settings: super::Settings) -> Result<AsyncSerial, serialport::Error> {
    panic!("not supported on windows yet")
}
