use core::time::Duration;
fn main() {
    let mut port = match serialport::new("/dev/ttyAMA0", 300)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => p,
        Err(_) => {
            panic!()
        }
    };
    let mut serial_buf: Vec<u8> = vec![0; 1000];


    loop{
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let data = serial_buf[..t].to_vec();

                println!("{:?}",data);
            }
            Err(_) => {}
        }
    }
}
