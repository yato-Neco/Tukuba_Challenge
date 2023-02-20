use crate::robot::config;
use crate::robot::setting::Settings;
use ::tui::backend::CrosstermBackend;
use ::tui::Terminal;
use flacon::{Event, FlaCon, Flags};
use getch;
use gps::{self, GPS};
use lidar::ydlidarx2;
use mytools::time_sleep;
use robot_gpio::Moter;
use rthred::{send, sendG, Rthd, RthdG};
use scheduler::Scheduler;
use tui;

use config::SenderOrders;

use std::io::Stdout;
use std::time::Duration;
use std::{
    cell::Cell,
    collections::HashMap,
    io::{stdout, Write},
    sync::mpsc::{self, Receiver, Sender},
};

pub struct KeyModule {
    pub moter_controler: Moter,
}

#[derive(Debug)]
pub struct KeyEvents {
    pub is_debug: bool,
    pub is_avoidance: bool,
    pub is_move: bool,
    pub is_trune: bool,
    pub is_emergency_stop_lv1: bool,
    pub is_emergency_stop_lv0: bool,
    pub is_emergency_stop_lv0_delay: bool,
    pub order: u32,
}

#[macro_export]
macro_rules! thread_variable {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_hashmap = std::collections::HashMap::new();
            $(

                let tmp  = std::sync::mpsc::channel::<u32>();

                temp_hashmap.insert($x,tmp);
            )*

            temp_hashmap

        }
    };
}

pub fn key() {
    //let mut terminal = tui::start();

    let setting_file = Settings::load_setting("./settings.yaml");

    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();

    let moter_controler = Moter::new(right_moter_pin, left_moter_pin);

    let module = KeyModule { moter_controler };

    let event = KeyEvents {
        is_debug: true,
        is_avoidance: false,
        is_move: false,
        is_trune: false,
        is_emergency_stop_lv1: false,
        is_emergency_stop_lv0: false,
        is_emergency_stop_lv0_delay: false,
        order: 0xfffffff,
    };

    let mut flag_controler = FlaCon::new(module, event);

    //flag_controler.event.is_move.set(true);

    flag_controler.add_fnc("is_stop", |flacn| {
        // is_move が false だったら呼び出す。
        if !flacn.event.is_move {
            //println!("{:x}",flacn.event.order.get());
        };
    });

    flag_controler.add_fnc("moter_control", |flacn| {
        let order = flacn.event.order;
        if order != config::NONE && !flacn.event.is_emergency_stop_lv0 {
            flacn.load_fnc("set_move");
            flacn.module.moter_controler.moter_control(order);
        }
    });

    flag_controler.add_fnc("debug", |flacn| if flacn.event.is_debug {});

    flag_controler.add_fnc("set_move", |flacn| {
        // order が前進をだったら is_move を true にする。
        let order = flacn.event.order;
        if order == config::EMERGENCY_STOP || order == config::STOP {
            flacn.event.is_move = false;
        } else {
            if !flacn.event.is_move && order == config::NONE {
                flacn.event.is_move = false;
            } else {
                if !flacn.event.is_emergency_stop_lv0 {
                    flacn.event.is_move = true;
                }
            }
        }
    });

    flag_controler.add_fnc("is_stop", |flacn| {
        // is_stop が false の時、呼び出す

        if !flacn.event.is_move {
            //println!("stop");
            flacn.module.moter_controler.moter_control(config::STOP);
        };
    });
    flag_controler.add_fnc("is_emergency_stop", |flacn| {
        // is_emergency_stop_lv0 が true の時、呼び出す
        if flacn.event.is_emergency_stop_lv0 {
            flacn.module.moter_controler.moter_control(config::STOP);
            //flacn.module.raspico_controler.write(config::STOP);
        };
    });

    flag_controler.add_fnc("emergency_stop", |flacn| {
        // is_emergency_stop_lv0 が false で尚且つ、
        // order が前進をだったら is_move を true にする。

        flacn.load_fnc("set_emergency_stop");

        if flacn.event.is_emergency_stop_lv0 {
            flacn
                .module
                .moter_controler
                .moter_control(config::EMERGENCY_STOP);
            flacn.event.is_move = false;
            flacn.event.order = config::EMERGENCY_STOP;
        } else {
            flacn.load_fnc("set_move");
        }
    });

    flag_controler.add_fnc("set_emergency_stop", |flacn| {
        // order が EMERGENCY_STOP だったら EMERGENCY_STOP の bool を反転にする。
        if flacn.event.order == config::EMERGENCY_STOP {
            flacn.event.is_move = false;
            flacn.event.is_emergency_stop_lv0 = !flacn.event.is_emergency_stop_lv0;
        };
    });

    let order = thread_variable!("key", "lidar");

    let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    let mut thread: HashMap<&str, fn(Sender<String>, SenderOrders)> =
        std::collections::HashMap::new();

    thread.insert("key", |panic_msg: Sender<String>, msg: SenderOrders| {
        Rthd::<String>::send_panic_msg(panic_msg);
        let setting_file = Settings::load_setting("./settings.yaml");

        let key_bind = setting_file.load_key_bind();
        loop {
            let order = input_key(key_bind);
            send(order, &msg);
            time_sleep(0, 5);
        }
    });

    //thread.insert("lidar", lidar);
    //slet (gps_sender, gps_receiver) = std::sync::mpsc::channel::<bool>();

    thread.insert("lidar", |panic_msg: Sender<String>, msg: SenderOrders| {
        Rthd::<String>::send_panic_msg(panic_msg);
        let setting_file = Settings::load_setting("./settings.yaml");
        let lidar_setting = setting_file.load_lidar();
        let mut lidar_port = match serialport::new(lidar_setting.0, lidar_setting.1)
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Eight)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(p) => p,
            Err(_) => panic!(),
        };

        let mut serial_buf: Vec<u8> = vec![0; 1500];

        loop {
            match lidar_port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    //let mut map_vec = Vec::new();
                    let mut data = serial_buf[..t].to_vec();
                    //println!("{:?}",data);
                    if data.len() > 11 {
                        let points = ydlidarx2(&mut data);

                        for point in points.iter() {
                            if point.1 < 90.0 {
                                //315 250
                                if 250.0 < point.0 && point.0 < 315.0 {
                                    //println!("{:?}",point);
                                    send(0, &msg);
                                } else {
                                }
                            } else {
                                if 250.0 < point.0 && point.0 < 315.0 {
                                } else {
                                    //send(1, &msg);
                                }
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    });

    Rthd::<String>::thread_generate(thread, &sendr_err_handles, &order);

    loop {
        match order.get("lidar").unwrap().1.try_recv() {
            Ok(e) => {
                flag_controler.event.is_emergency_stop_lv0 = e == 0;
                //flag_controler.event.order = e;
                //println!("E:{:x}", flag_controler.event.order);
                flag_controler.load_fnc("emergency_stop");
                flag_controler.load_fnc("moter_control");
            }
            Err(_) => {}
        };

        match order.get("key").unwrap().1.try_recv() {
            Ok(e) => {
                if e == config::BREAK {
                    let flag = flag_controler.module.moter_controler.reset();
                    if flag {
                        break;
                    }
                } else {
                    flag_controler.event.order = e;
                    flag_controler.event.order = e;
                    println!("{:x}", flag_controler.event.order);

                    //flag_controler.load_fnc("set_emergency_stop");
                    flag_controler.load_fnc("emergency_stop");
                    flag_controler.load_fnc("moter_control");
                    //flag_controler.load_fnc("is_stop");
                    //flag_controler.load_fnc("is_emergency_stop");
                }
            }
            Err(_) => {}
        };

        time_sleep(0, 5);
    }

    //terminal.clear().unwrap();
}

pub fn input_key(key_bind: [u32; 4]) -> u32 {
    let key = getch::Getch::new();
    let key_order_u8 = key.getch().unwrap();
    //println!("{}", key_order_u8);

    let order = match key_order_u8 {
        119 => {
            // w
            //0x1FDDFFFF
            key_bind[0]
        }
        97 => {
            // a
            //0x1F6DFFFF
            key_bind[1]
        }
        115 => {
            // s
            //0x1F66FFFF
            key_bind[2]
        }
        100 => {
            // d
            //0x1FD6FFFF
            key_bind[3]
        }
        32 => {
            // stop
            config::STOP
        }
        3 => {
            // break
            config::BREAK
        }
        49 => config::EMERGENCY_STOP,

        _ => config::NONE,
    };
    order
}
