use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
    time::{Duration, Instant},
};

use crate::thread_variable;
use mytools::{mic_sleep, ms_sleep, time_sleep, Xtools};
use robot_gpio::Moter;
use rthred::{sendG, Rthd};

use crate::robot::{
    config::{self, SenderOrders},
    setting::Settings,
};

use wt901::WT901;

pub fn test() {
    let setting_file = Settings::load_setting("./settings.yaml");

    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();

    let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);

    let mut thread: HashMap<&str, fn(Sender<String>, SenderOrders)> =
        std::collections::HashMap::new();
    let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    let opcode = thread_variable!("operator");

    //thread.insert("operator", operator);

    //1s 125ms
    //1s 40~50ms ????

    let in_ms = 180 + 5; //1s
    let in_ms = 1300000; //185000
                         // in debug 1344000
                         // in rlease
    let out_ms = 105;

    let mut wt901_port = match serialport::new("/dev/ttyUSB0", 9600)
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

    let mut wt901_serial_buf: Vec<u8> = vec![0; 2000];


    let mut wt901_serial = WT901::new();

    loop {

        match wt901_port.read(wt901_serial_buf.as_mut_slice()) {
            Ok(t) => {
                let data = wt901_serial_buf[..t].to_vec();
                wt901_serial.cope_serial_data(data);
                wt901_serial.z_aziment();
                println!("{:?}",wt901_serial.aziment.2);
            }

            Err(_) => {}
        }


        moter_controler.moter_control(0x1F6DFFFF);
        

        if wt901_serial.aziment.2 < -90.0 {
            moter_controler.moter_control(config::STOP);
            //break;
        }

        //ms_sleep(10);
    }
}

fn operator(panic_msg: Sender<String>, msg: SenderOrders) {
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

    /*
        match microbit_port.read(microbit_serial_buf.as_mut_slice()) {
            Ok(t) => {
                let data = microbit_serial_buf[..t].to_vec();

                println!("{:?}", data);

                azimuth = 0;
            }

            Err(_) => {}
        }
    */

    loop {
        sendG(config::FRONT, &msg);

        /*
        if azimuth > 90 {
            moter_controler.moter_control(config::STOP);
        }
        */
    }
}

fn serial() {
    time_sleep(0, 10);
}

#[test]
fn s() {
    let v2 = [0.0, 1.0, 0.0, 0.0, -1.0];
    let mut v = 0.0;

    let mut timer = Scheduler::start();

    for vv in v2 {
        let t = timer.end();
        v += vv * t;
        println!("{} {}", v.roundf(10), t);
        timer = Scheduler::start();
        time_sleep(2, 000);
    }
}

#[derive(Debug, Clone)]
pub struct Scheduler {
    start_time: Instant,
}

impl Scheduler {
    // カウントを開始する。
    pub fn start() -> Scheduler {
        let now_time = Instant::now();

        Scheduler {
            start_time: now_time,
        }
    }

    pub fn end(&self) -> f64 {
        // 二つ目のカウントが始まって終わった時の時間を、一つ目のカウントでは無かったことにする。
        self.start_time.elapsed().as_secs_f64()
    }
}
