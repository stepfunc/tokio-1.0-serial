// use serialport::SerialPort;
use tokio::io::DuplexStream;

pub type AsyncSerial = DuplexStream;

pub fn open(_path: &str, _settings: super::Settings) -> std::io::Result<AsyncSerial> {
    unimplemented!()
    /*
    let read_port = settings.build(serialport::new(path, settings.baud_rate)).open()?;
    let write_port = read_port.try_clone()?;

    let (client, server) = tokio::io::duplex(128);

    Ok(client)

     */
}

/*
fn run_read(runtime: tokio::runtime::Handle, mut port: Box<dyn SerialPort>, mut duplex: DuplexStream) -> Result<(), std::io::Error> {
    let mut buffer: [u8; 128] = [0; 128];

    loop {
        let count = port.read(&mut buffer)?;
        if count == 0 {
            return Err(std::io::ErrorKind::UnexpectedEof.into());
        }

        let guard = runtime.enter();
        runtime.spawn()
        duplex.write_all(buffer[0..count])
    }
}
*/
