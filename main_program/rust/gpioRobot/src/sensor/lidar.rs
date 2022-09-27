use crate::robot::settings::Settings;
use crate::xtools::{warning_msg};
use std::time::Duration;
use yaml_rust::Yaml;
use ydlidarx2_rs;

fn lidar(path:&str,settings_yaml:&Yaml ) {

    let mut port = match serialport::new(settings_yaml["Robot"]["Lidar"]["port"][0].as_str().unwrap(), 115200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => (p),
        Err(_) => (panic!()),
    };

    let mut serial_buf: Vec<u8> = vec![0_u8; 500];

    let threshold:f64 = Settings::load_setting(path)["Robot"]["lidar"]["threshold"][0]
        .as_f64()
        .unwrap();

    let mut countt:usize  = 0;
    

    let mut countt2:usize  = 0;

    let mut flag0 = false;
    let mut flag1 = false;


    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let mut data = serial_buf[..t].to_vec();


                let points =  ydlidarx2_rs::ydlidarx2(&mut data);
                
                for i in points {
                    if i.0 >= 45.0 && i.0 <= 235.0 && i.1 < threshold {
                        println!("{}度 {}cm", i.0, i.1);

                        countt+=1;
                        countt2 = 0;
                    }

                    if i.0 >= 45.0 && i.0 <= 235.0 && i.1 >= threshold {
                        countt2+=1;
                        countt = 0;
                    }

                    if countt == 1  || countt2 == 1{

                            
                    }

                    if countt > 2 || countt2 > 2 {
                        countt = 2;
                        countt2 = 2;
                    }

                    
                }
            }

            Err(_) => {
                warning_msg("");
            }
        }
    }
}

#[test]
fn test() {
    let mut test_data: [u8; 90] = [
        170, 85, 134, 40, 237, 112, 199, 142, 202, 217, 232, 36, 216, 36, 196, 36, 132, 36, 60, 36,
        24, 36, 0, 0, 0, 0, 170, 91, 6, 3, 240, 2, 244, 2, 252, 2, 0, 0, 174, 3, 78, 3, 62, 3, 50,
        3, 48, 3, 86, 3, 0, 0, 0, 0, 0, 0, 0, 0, 42, 11, 108, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let points =  ydlidarx2_rs::ydlidarx2(&mut test_data);

    println!("{:?}",points);

}