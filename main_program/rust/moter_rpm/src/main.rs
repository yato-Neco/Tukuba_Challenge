use core::time::Duration;

use moter_rpm::XtoolsBool;
mod lib;

struct  GNRMC {
    utc:&'static str,
    status:&'static str,
    lat:&'static str,
    lot:&'static str,
    speed:&'static str,
    
}


fn main() {
    let mut port = match serialport::new("COM3", 115200)
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

                    let vec: Vec<&str> = gps_data.split("$").collect();
                    //println!("{:?}",vec);
                    for (i, data) in vec.iter().enumerate() {
                        match data.find("GNRMC") {
                            Some(p) => {
                                let gnrmc = vec[i].trim();
                                let g = gnrmc.split(",").collect::<Vec<&str>>();
                                if  g.len() > 12 {
                                    
                                

                                    let is_fix = g[2] == "A";
                                    let lat = g[3].parse::<f64>().unwrap_or(0.0) / 100.0;
                                    let lot = g[5].parse::<f64>().unwrap_or(0.0) / 100.0;

                                    println!("{:?}", g);


                                }

                            }

                            None => {}
                        }
                    }

                    tmp_vec.clear();
                }
            }
            Err(_) => {}
        }
    }
}
