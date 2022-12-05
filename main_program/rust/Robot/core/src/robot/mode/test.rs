use crate::robot::config;
use crate::robot::setting::Settings;
use lidar::ydlidarx2;
use ::tui::backend::CrosstermBackend;
use ::tui::Terminal;
use flacon::{Event, FlaCon, Flags};
use getch;
use gps::{self, GPS};
use mytools::time_sleep;
use robot_gpio::Moter;
use robot_serialport::RasPico;
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
use crate::thread_variable;

use super::key::input_key;


pub struct KeyModule {
    pub moter_controler: Moter,
}

#[derive(Debug)]
struct TestModule {
    //pub moter_controler: Moter,
}

#[derive(Debug)]
pub struct TestEvents {
    pub is_debug: bool,
    pub is_avoidance: bool,
    pub is_move: bool,
    pub is_trune: bool,
    pub is_emergency_stop_lv1: bool,
    pub is_emergency_stop_lv0: bool,
    pub order: u32,
}

pub fn test() {
    let setting_file = Settings::load_setting("./settings.yaml");
        let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();
        let operation = setting_file.load_move_csv();
        //let (gps_port, gps_rate, gps_buf_size) = setting_file.load_gps_serial();
        //let (rp_port, rp_rate) = setting_file.load_raspico();
        //let raspico_controler = RasPico::new(&rp_port, rp_rate);
        let moter_controler = Moter::new(right_moter_pin, left_moter_pin);

        let gps = GPS::new(true);


        // Lidar も
        let module = TestModule {
            //moter_controler,
        };
        let event = TestEvents {
            is_debug: true,
            is_avoidance: false,
            is_move: false,
            is_trune: false,
            is_emergency_stop_lv1: false,
            is_emergency_stop_lv0: false,
            order: 0xfffffff,
        };

        let mut flag_controler = FlaCon::new(module, event);
        flag_controler.add_fnc("is_stop", |flacn| {
            // is_move が false だったら呼び出す。
            if !flacn.event.is_move {
                //println!("{:x}",flacn.event.order.get());
            };
        });
        flag_controler.add_fnc("moter_control", |flacn| {
            let order = flacn.event.order;
            if order != config::NONE {
                //flacn.module.moter_controler.moter_control(order);
            }
        });
        flag_controler.add_fnc("move", |flacn| {
            // is_move が true だったら呼び出す。
            if flacn.event.is_move {
                flacn.load_fnc("moter_control");
                //println!("is_move");
            };
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
                    flacn.event.is_move = true;
                }
            }
        });
        flag_controler.add_fnc("is_stop", |flacn| {
            // is_stop が false の時、呼び出す
            if !flacn.event.is_move {
                //println!("stop");
                //flacn.module.moter_controler.pwm_all_clean();
            };
        });
        flag_controler.add_fnc("is_emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が true の時、呼び出す
            if flacn.event.is_emergency_stop_lv0 {
                //flacn.module.moter_controler.pwm_all_clean();
            };
        });
        flag_controler.add_fnc("emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が false で尚且つ、
            // order が前進をだったら is_move を true にする。
            if flacn.event.is_emergency_stop_lv0 {
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

        let order = thread_variable!("lidar", "moter");
        let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
            mpsc::channel();

        let mut thread: HashMap<&str, fn(Sender<String>, SenderOrders)> =
            std::collections::HashMap::new();

        let (main_sender, main_receiver) = std::sync::mpsc::channel::<u32>();
        let (moter_sender, moter_receiver) = std::sync::mpsc::channel::<(u32, u32)>();

        RthdG::_thread_generate_return_sender(
            "moter",
            &sendr_err_handles,
            moter_receiver,
            moter_controler,
            |panic_msg, moter_receiver, moter_controler| {
                Rthd::<String>::send_panic_msg(panic_msg);
                #[derive(Debug)]
                pub struct TestOderEvents {
                    pub is_emergency_stop_lv0: bool,
                    pub order: (u32, u32),
                    pub is_interruption: bool,
                    pub order1_vec: Vec<(u32, u32)>,
                    pub order0_vec: Vec<(u32, u32)>,
                }
                //let mut isexecution = false;

                let event = TestOderEvents {
                    is_emergency_stop_lv0: false,
                    order: (config::STOP, 0),
                    is_interruption: false,
                    order1_vec: Vec::<(u32, u32)>::new(),
                    order0_vec: Vec::<(u32, u32)>::new(),
                };

                let mut scheduler = Scheduler::start();
                struct Test {
                    scheduler: Scheduler,
                    moter_controler: Moter,
                }
                let module = Test {
                    scheduler,
                    moter_controler,
                };

                let mut order_controler = FlaCon::new(module, event);
                ////order_vec.push((0xffffffff,0));
                let mut stoptime: i128 = 0;

                order_controler.add_fnc("emergency_stop", |flacn| {
                    if flacn.event.order.0 == config::EMERGENCY_STOP {
                        flacn.event.is_emergency_stop_lv0 = !flacn.event.is_emergency_stop_lv0;
                    }
                });

                order_controler.add_fnc("order", |flacn| {
                    let order_28 = (flacn.event.order.0 & 0xF0000000) >> 28_u8;
                    match order_28 {
                        0 => flacn.event.order0_vec.push(flacn.event.order),
                        1 => flacn.event.order1_vec.push(flacn.event.order),
                        _ => {}
                    }
                });

                order_controler.add_fnc("order0_vec", |flacn| {
                    // スケジュラー

                    let order0_vec_len = flacn.event.order0_vec.len();

                    if order0_vec_len == 1 {
                        flacn.event.is_interruption = !flacn.event.is_interruption;

                        if flacn.event.is_interruption {
                            flacn.module.scheduler.add_time_counter();
                        } else {
                            flacn.module.scheduler.end();
                        }
                        //moter_controler.moter_control(config::EMERGENCY_STOP);
                        println!("{}", flacn.module.scheduler.nowtime());
                        flacn.module.moter_controler.moter_control(config::EMERGENCY_STOP);

                        flacn.event.order0_vec.remove(0);
                    }
                });

                let mut now_time = 0;

                loop {
                    match moter_receiver.try_recv() {
                        Ok(e) => {
                            order_controler.event.order = e;
                            order_controler.load_fnc("emergency_stop");
                            order_controler.load_fnc("order");
                        }
                        Err(_) => {}
                    };

                    order_controler.load_fnc("order0_vec");

                    /*
                    if order_controler.event.is_interruption {
                        continue;
                    }
                    */

                    if !order_controler.event.is_interruption {
                        match order_controler.event.order1_vec.get(0) {
                            Some(e) => {
                                // 誤差 ±10ms
                                now_time = order_controler.module.scheduler.nowtime();

                                if now_time - 2 >= stoptime && stoptime <= now_time + 2 {
                                    stoptime = stoptime + e.1 as i128;

                                    println!("{:x} {}", e.0, e.1);

                                    order_controler.module.moter_controler.moter_control(e.0);

                                    order_controler.event.order1_vec.remove(0);
                                }
                            }

                            None => {}
                        }
                    }

                    time_sleep(0, 1);
                }
            },
        );

        // time_sleep があると、その他のモジュールに影響を与えるのでモーター制御は別制御で
        for i in 0..operation.len() {
            //println!("{} {:x}", i, operation[i].0);
            let order = operation[i].0;
            flag_controler.event.order = order;
            sendG(operation[i], &moter_sender);
        }

        let key_bind = setting_file.load_key_bind();


        loop {
            let key_order = input_key(key_bind);
            if key_order == config::BREAK {
                break;
            }
            if key_order == config::EMERGENCY_STOP {
                moter_sender.send((config::EMERGENCY_STOP, 0)).unwrap();
            }

            //time_sleep(3, 0);

            time_sleep(0, 6)
        }
}