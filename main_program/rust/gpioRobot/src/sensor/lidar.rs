use std::io;
use std::io::Write;
use std::time::Duration;

#[test]
fn lider() {
    let mut port = serialport::new("COM4", 115200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyUSB", e);
            ::std::process::exit(1);
        });

    let mut serial_buf: Vec<u8> = vec![0; 1500];
    //1 2 3 4 5 6

    //let mut tmp = [];

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                //println!("{:?}", &serial_buf[..t]);
                u8_tou16(&serial_buf[..t]);
            }
            Err(_e) => {}
        }
    }
}


fn u8_tou16(title:&[u8]){
    let title: Vec<u16> = title
    .chunks_exact(2)
    .into_iter()
    .map(|a| u16::from_ne_bytes([a[0], a[1]]))
    .collect();
    let title = title.as_slice();
    println!("{:?}",title);
}