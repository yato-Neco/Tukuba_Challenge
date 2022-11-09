use lidar::ydlidarx2;
use std::time::Duration;

#[test]
fn lider() {
    let mut port = match serialport::new("COM4", 115200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => (p),
        Err(_) => (panic!()),
    };
 
    let mut serial_buf: Vec<u8> = vec![0; 1500];

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let mut data = serial_buf[..t].to_vec();
                let points = ydlidarx2(&mut data);
                for i in points {

                    if i.0 >= 165.0 && i.0 <= 195.0 && i.1 < 4.5 {
                        println!("{}度 {}cm", i.0, i.1);
                    }

                }
            }

            Err(_) => {}
        }
    }
}


