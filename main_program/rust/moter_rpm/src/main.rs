use core::time::Duration;

use moter_rpm::XtoolsBool;
mod lib;

struct GNRMC {
    utc: &'static str,
    status: &'static str,
    lat: &'static str,
    lot: &'static str,
    speed: &'static str,
}

fn main() {
    let mut port = match serialport::new("/dev/ttyAMA0", 115200)
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
    let mut tmp_vec: Vec<u8> = Vec::new();

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let mut data = serial_buf[..t].to_vec();
                tmp_vec.append(&mut data);

                if tmp_vec.len() > 350 {
                    let gps_data = String::from_utf8_lossy(&tmp_vec).to_string();

                    let vec: Vec<&str> = gps_data.split("$GPGGA").collect();
                    match vec.get(1) {
                        Some(i) => {
                            let tmp: Vec<&str> = i.split("\r\n").collect();
                            let tmp2: Vec<&str> = tmp[0].split(",").collect();
                            if tmp2.len() > 14 {
                                let lat = tmp2[2].parse::<f64>().unwrap_or(0.0) / 100.0;
                                let lot = tmp2[4].parse::<f64>().unwrap_or(0.0) / 100.0;
                                let lat_tmp = ((lat - lat.floor()) * 100.0) / 60.0;
                                let lot_tmp = ((lot - lot.floor()) * 100.0) / 60.0;

                                println!(
                                    "{:?} {:?},{:?}",
                                    tmp2[6],
                                    lat.floor() + lat_tmp,
                                    lot.floor() + lot_tmp,
                                );
                            }
                        }
                        None => {}
                    }

                    tmp_vec.clear();
                }
            }
            Err(_) => {}
        }
    }
}
