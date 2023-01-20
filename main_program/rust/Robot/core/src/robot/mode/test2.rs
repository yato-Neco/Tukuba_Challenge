use std::{time::{Duration, Instant}, collections::HashMap, sync::mpsc::{Sender, Receiver, self}};

use mytools::{time_sleep, Xtools};
use robot_gpio::Moter;
use rthred::{Rthd, sendG};
use crate::thread_variable;

use crate::robot::{setting::Settings, config::{self, SenderOrders}};

pub fn test() {
    let setting_file = Settings::load_setting("./settings.yaml");

    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();

    let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);


    let mut thread: HashMap<&str, fn(Sender<String>, SenderOrders)> =
        std::collections::HashMap::new();
        let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) = mpsc::channel();
    
    let opcode = thread_variable!("operator");


    thread.insert("operator", operator);


    Rthd::<String>::thread_generate(thread, &sendr_err_handles, &opcode);


    


    let mut azimuth:u32 = 0;
    moter_controler.moter_control(0x1FA4FFFF);

    loop {

        
        match opcode.get("operator").unwrap().1.try_recv() {
            Ok(e) => {
                moter_controler.moter_control(e);
            }
            Err(_) => {}
        };


        time_sleep(0, 10);

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
    let v2 = [0.0,1.0,0.0,0.0,-1.0];
    let mut v = 0.0; 

    let mut timer = Scheduler::start();

    
    for vv in v2 {
        let t = timer.end();
        v +=  vv * t;
        println!("{} {}",v.roundf(10),t);
        timer = Scheduler::start();
        time_sleep(2, 000);

    }


}

#[derive(Debug,Clone)]
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