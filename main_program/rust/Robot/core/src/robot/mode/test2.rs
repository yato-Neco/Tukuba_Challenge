use std::time::Duration;

use mytools::time_sleep;
use robot_gpio::Moter;

use crate::robot::{setting::Settings, config};

pub fn test() {
    let setting_file = Settings::load_setting("./settings.yaml");

    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();

    let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);

    let mut microbit_port = match serialport::new("COM5", 9600)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => p,
        Err(_) => panic!(),
    };

    let mut microbit_serial_buf: Vec<u8> = vec![0; 1000];

    let mut azimuth:u32 = 0;
    moter_controler.moter_control(0x1FA4FFFF);

    loop {


        match microbit_port.read(microbit_serial_buf.as_mut_slice()) {
            Ok(t) => {
                let data = microbit_serial_buf[..t].to_vec();
    
                println!("{:?}", data);

                azimuth = 0;
            }
    
            Err(_) => {}
        }


        if azimuth > 90 {
            moter_controler.moter_control(config::STOP);
        }

        time_sleep(0, 10);

    }
}


fn serial() {
    time_sleep(0, 10);
}
