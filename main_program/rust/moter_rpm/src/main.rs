use core::time::Duration;
fn main() {
    let mut port = match serialport::new("/dev/ttyAMA0", 9600)
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
    let mut serial_buf: Vec<u8> = vec![0; 2000];
    let mut tmp_vec:Vec<u8> = Vec::new();

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let mut data = serial_buf[..t].to_vec();
                tmp_vec.append(&mut data);
                if tmp_vec.len() > 800 {
                    let gps_data = String::from_utf8_lossy(&tmp_vec).to_string();
                    println!("{}:",gps_data);
                  
                    tmp_vec.clear();
                }
                


            }
            Err(_) => {}
        }
    }
}
