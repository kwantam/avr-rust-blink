use serial2::{CharSize, FlowControl, Parity, SerialPort, Settings, StopBits};
use std::io::{self, Write};
use std::time::Duration;

fn main() -> Result<(), io::Error> {
    let file = match std::env::args().nth(1) {
        Some(file) => file,
        None => {
            println!("USAGE: serx /dev/something");
            std::process::exit(1);
        }
    };

    let mut buf = [0u8; 1024];
    let mut stdout = io::stdout().lock();

    let new_port = || {
        // 38400 8n1 with no flow control
        let port = SerialPort::open(&file, |mut s: Settings| {
            s.set_raw();
            s.set_baud_rate(38400)?;
            s.set_char_size(CharSize::Bits8);
            s.set_parity(Parity::None);
            s.set_stop_bits(StopBits::One);
            s.set_flow_control(FlowControl::None);
            Ok(s)
        })?;
        port.discard_buffers()?;
        Ok::<_, io::Error>(port)
    };

    loop {
        match new_port() {
            Ok(port) => 'inner: loop {
                match port
                    .read(&mut buf)
                    .and_then(|size| stdout.write_all(&buf[..size]))
                {
                    Ok(_) => (),
                    Err(e) => {
                        println!("==== ERROR (read): {e}");
                        break 'inner;
                    }
                }
            },
            Err(e) => {
                println!("==== ERROR (open): {e}");
            }
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
