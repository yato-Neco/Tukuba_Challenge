use std::time::Duration;
use std::io;
use std::io::Write;

#[test]

fn lider(){
    let mut port = serialport::new("/dev/ttyUSB0", 115200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyUSB", e);
            ::std::process::exit(1);
    });


    let mut serial_buf: Vec<u8> = vec![0; 1000];
    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
            Err(_e) => {},
        }
    }


}