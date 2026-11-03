use serial2::{SerialPort, CharSize, StopBits, Parity, FlowControl, Settings};
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    // 38400 8n1 with no flow control
    let port = SerialPort::open("/dev/cu.usbserial-110", |mut s: Settings| {
        s.set_raw();
        s.set_baud_rate(38400)?;
        s.set_char_size(CharSize::Bits8);
        s.set_parity(Parity::None);
        s.set_stop_bits(StopBits::One);
        s.set_flow_control(FlowControl::None);
        Ok(s)
    })?;
    port.discard_buffers()?;

    let mut buf = [0u8; 1024];
    let mut stdout = io::stdout().lock();
    loop {
        port.read(&mut buf)?;
        stdout.write_all(&buf)?;
    }
}
