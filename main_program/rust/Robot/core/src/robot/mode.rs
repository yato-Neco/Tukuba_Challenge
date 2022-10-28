use crate::xtools::{time_sleep, warning_msg};
use flacon::{Event, FlaCon, Flags};
use getch;
use gps::{self, GPS};
use robot_gpio::Moter;

use super::tui;
use super::{
    config::{self, SenderOrders},
    rthred::{Rthd, RthdG},
    setting::Settings,
};

use std::{
    cell::Cell,
    collections::HashMap,
    io::{stdout, Write},
    sync::mpsc::{self, Receiver, Sender},
};

pub struct Mode {}

pub struct AutoModule {
    pub moter_controler: Moter,
    pub gps: GPS,
    // pub slam: SLAM
}
pub struct KeyModule {
    pub moter_controler: Moter,
}
pub struct TestModule {
    pub moter_controler: Moter,
    pub gps: GPS,
}

#[derive(Debug)]
pub struct AutoEvents {
    pub is_debug: bool,
    pub is_avoidance: bool,
    pub is_break: bool,
    pub is_move: Cell<bool>,
    pub is_trune: Cell<bool>,
    pub is_emergency_stop_lv1: Cell<bool>,
    pub is_emergency_stop_lv0: Cell<bool>,
    pub order: Cell<u32>,
    pub order_history: Vec<u32>,
    pub latlot: (f64, f64),
}

/// フラグのイベント一覧
#[derive(Debug)]
pub struct KeyEvents {
    pub is_debug: bool,
    pub is_avoidance: bool,
    pub is_move: Cell<bool>,
    pub is_trune: Cell<bool>,
    pub is_emergency_stop_lv1: Cell<bool>,
    pub is_emergency_stop_lv0: Cell<bool>,
    pub order: Cell<u32>,
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

/// ロボットのモード構造体
impl Mode {
    pub fn auto() {
        let mut terminal = tui::start();

        let setting_file = Settings::load_setting("./settings.yaml");

        let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();

        let (port, rate, buf_size) = setting_file.load_gps_serial();

        let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);

        let mut gps = GPS::new(port.as_str(), rate, buf_size);

        //モジュールをflag内で扱うための構造体
        let module = AutoModule {
            moter_controler,
            gps,
        };

        // Robot の event(flag)管理
        let event = AutoEvents {
            is_debug: true,
            is_avoidance: true,
            is_break: false,
            is_move: Cell::new(false),
            is_trune: Cell::new(false),
            is_emergency_stop_lv1: Cell::new(false),
            is_emergency_stop_lv0: Cell::new(false),
            order: Cell::new(0xfffffff),
            order_history: Vec::new(),
            latlot: (0.0, 0.0),
        };

        // mut を外したい
        let mut flag_controler = FlaCon::new(module, event);

        //flag_controler.event.is_move.set(true);

        flag_controler.add_fnc("is_stop", |flacn| {
            // is_move が false だったら呼び出す。
            if !flacn.event.is_move.get() {

                //println!("{:x}",flacn.event.order.get());
            };
        });

        flag_controler.add_fnc("moter_control", |flacn| {
            let order = flacn.event.order.get();
            if order != config::None {
                flacn.module.moter_controler.moter_control(order);
            }
        });

        flag_controler.add_fnc("move", |flacn| {
            // is_move が true だったら呼び出す。
            if flacn.event.is_move.get() {
                flacn.load_fnc("moter_control");
                //println!("is_move");
            };
        });

        flag_controler.add_fnc("debug", |flacn| if flacn.event.is_debug {});

        flag_controler.add_fnc("set_move", |flacn| {
            // order が前進をだったら is_move を true にする。
            let order = flacn.event.order.get();
            if order == config::EMERGENCY_STOP || order == config::STOP {
                flacn.event.is_move.set(false);
            } else {
                if !flacn.event.is_move.get() && order == config::None {
                    flacn.event.is_move.set(false);
                } else {
                    flacn.event.is_move.set(true);
                }
            }
        });

        flag_controler.add_fnc("is_stop", |flacn| {
            // is_stop が false の時、呼び出す

            if !flacn.event.is_move.get() {
                //println!("stop");
                flacn.module.moter_controler.pwm_all_clean();
            };
        });
        flag_controler.add_fnc("is_emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が true の時、呼び出す
            if flacn.event.is_emergency_stop_lv0.get() {
                flacn.module.moter_controler.pwm_all_clean();
            };
        });

        flag_controler.add_fnc("emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が false で尚且つ、
            // order が前進をだったら is_move を true にする。
            if flacn.event.is_emergency_stop_lv0.get() {
            } else {
                flacn.load_fnc("set_move");
            }
        });

        flag_controler.add_fnc("set_emergency_stop", |flacn| {
            // order が EMERGENCY_STOP だったら EMERGENCY_STOP の bool を反転にする。
            if flacn.event.order.get() == config::EMERGENCY_STOP {
                flacn.event.is_move.set(false);
                flacn
                    .event
                    .is_emergency_stop_lv0
                    .set(!flacn.event.is_emergency_stop_lv0.get());
            };
        });

        flag_controler.add_fnc("gps_nav", |flacn| {
            // GPS Nav 終了フラグなど
            let mut gps = &mut flacn.module.gps;
            let isend = gps.nav();
            //print!("{}",isend);
            flacn.event.is_break = !isend;

            // gps
        });

        let (gps_sender, gps_receiver) = std::sync::mpsc::channel::<String>();
        let order = thread_variable!("key", "lidar");

        let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
            mpsc::channel();

        let mut thread: HashMap<&str, fn(Sender<String>, SenderOrders)> =
            std::collections::HashMap::new();

        thread.insert("key", |panic_msg: Sender<String>, msg: SenderOrders| {
            Rthd::<String>::send_panic_msg(panic_msg);
            loop {
                let order = Mode::input_key();
                msg.send(order).unwrap();
            }
        });

        /*
        thread.insert("gps", |panic_msg: Sender<String>, msg: SenderOrders| {
            Rthd::<String>::send_panic_msg(panic_msg);
            let order =  GPS::serial();
            time_sleep(1, 500);
            print!("{}",0x1FEEFFFF);
            msg.send(0x1FEEFFFF).unwrap();
        });
        */

        Rthd::_thread_generate(
            "gps",
            &sendr_err_handles,
            gps_sender,
            |panic_msg, gps_sender| {
                Rthd::<String>::send_panic_msg(panic_msg);
                GPS::serial("COM4", 115200, 500, gps_sender);
                //print!("gps");
            },
        );
        Rthd::<String>::thread_generate(thread, &sendr_err_handles, &order);

        loop {
            // Lidar 後に SLAM
            match order.get("lidar").unwrap().1.try_recv() {
                Ok(e) => {
                    flag_controler.event.order.set(e);
                    flag_controler.load_fnc("set_emergency_stop");
                }
                Err(_) => {}
            };

            // GPS
            match gps_receiver.try_recv() {
                Ok(e) => {
                    flag_controler.module.gps.original_nowpotion = e;
                    flag_controler.module.gps.parser("".to_owned());
                }
                Err(_) => {}
            }

            flag_controler.load_fnc("gps_nav");

            // Key
            match order.get("key").unwrap().1.try_recv() {
                Ok(e) => {
                    if e == config::BREAK {
                        break;
                    } else if e == config::EMERGENCY_STOP {
                        flag_controler.event.order.set(e);
                        flag_controler.load_fnc("set_emergency_stop");
                        flag_controler.load_fnc("emergency_stop");
                        flag_controler.load_fnc("is_emergency_stop");
                    }
                }
                Err(_) => {}
            };

            terminal
                .draw(|f| {
                    tui::auto_ui(f, &flag_controler);
                })
                .unwrap();
            //flag_controler.load_fnc("debug");

            if flag_controler.event.is_break {
                break;
            }

            time_sleep(0, 1);
        }

        //tui::end();
    }

    /// test mode
    pub fn test() {
        let mut terminal = tui::start();
        let setting_file = Settings::load_setting("./settings.yaml");
        let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();
        let mut operation = setting_file.load_move_csv();
        let (port, rate, buf_size) = setting_file.load_gps_serial();
        let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);
        let mut gps = GPS::new(port.as_str(), rate, buf_size);
        let moter_controler_clone = moter_controler.clone();

        // Lidar も
        let module = TestModule {
            moter_controler,
            gps,
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
            if order != config::None {
                flacn.module.moter_controler.moter_control(order);
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
                if !flacn.event.is_move && order == config::None {
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
                flacn.module.moter_controler.pwm_all_clean();
            };
        });
        flag_controler.add_fnc("is_emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が true の時、呼び出す
            if flacn.event.is_emergency_stop_lv0 {
                flacn.module.moter_controler.pwm_all_clean();
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

        let (main_sender, main_receiver) = std::sync::mpsc::channel::<(u32)>();
        let (moter_sender, moter_receiver) = std::sync::mpsc::channel::<(Sender<u32>)>();
        RthdG::_thread_generate_return_sender(
            "name",
            &sendr_err_handles,
            moter_sender,
            moter_controler_clone,
            |panic_msg, moter_sender, moter_controler| {
                Rthd::<String>::send_panic_msg(panic_msg);
            },
        );

        // time_sleep があると、その他のモジュールに影響を与えるのでモーター制御は別制御で
        for i in 0..operation.len() {
            println!("{} {:x}", i, operation[i].0);

            let order = operation[i].0;

            flag_controler.event.order = order;

            time_sleep(0, operation[i].1 as u64);
        }

        loop {
            let key_order = Mode::input_key();

            if key_order == config::BREAK {
                break;
            }
            time_sleep(0, 6)
        }
    }

    pub fn key() {
        let mut terminal = tui::start();

        let setting_file = Settings::load_setting("./settings.yaml");

        let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();

        let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);

        let module = KeyModule { moter_controler };

        let event = KeyEvents {
            is_debug: true,
            is_avoidance: false,
            is_move: Cell::new(false),
            is_trune: Cell::new(false),
            is_emergency_stop_lv1: Cell::new(false),
            is_emergency_stop_lv0: Cell::new(false),
            order: Cell::new(0xfffffff),
        };

        // mut を外したい
        let mut flag_controler = FlaCon::new(module, event);

        //flag_controler.event.is_move.set(true);

        flag_controler.add_fnc("is_stop", |flacn| {
            // is_move が false だったら呼び出す。
            if !flacn.event.is_move.get() {

                //println!("{:x}",flacn.event.order.get());
            };
        });

        flag_controler.add_fnc("moter_control", |flacn| {
            let order = flacn.event.order.get();
            if order != config::None {
                flacn.module.moter_controler.moter_control(order);
            }
        });

        flag_controler.add_fnc("move", |flacn| {
            // is_move が true だったら呼び出す。
            if flacn.event.is_move.get() {
                flacn.load_fnc("moter_control");
                //println!("is_move");
            };
        });

        flag_controler.add_fnc("debug", |flacn| if flacn.event.is_debug {});

        flag_controler.add_fnc("set_move", |flacn| {
            // order が前進をだったら is_move を true にする。
            let order = flacn.event.order.get();
            if order == config::EMERGENCY_STOP || order == config::STOP {
                flacn.event.is_move.set(false);
            } else {
                if !flacn.event.is_move.get() && order == config::None {
                    flacn.event.is_move.set(false);
                } else {
                    flacn.event.is_move.set(true);
                }
            }
        });

        flag_controler.add_fnc("is_stop", |flacn| {
            // is_stop が false の時、呼び出す

            if !flacn.event.is_move.get() {
                //println!("stop");
                flacn.module.moter_controler.pwm_all_clean();
            };
        });
        flag_controler.add_fnc("is_emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が true の時、呼び出す
            if flacn.event.is_emergency_stop_lv0.get() {
                flacn.module.moter_controler.pwm_all_clean();
            };
        });

        flag_controler.add_fnc("emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が false で尚且つ、
            // order が前進をだったら is_move を true にする。
            if flacn.event.is_emergency_stop_lv0.get() {
            } else {
                flacn.load_fnc("set_move");
            }
        });

        flag_controler.add_fnc("set_emergency_stop", |flacn| {
            // order が EMERGENCY_STOP だったら EMERGENCY_STOP の bool を反転にする。
            if flacn.event.order.get() == config::EMERGENCY_STOP {
                flacn.event.is_move.set(false);
                flacn
                    .event
                    .is_emergency_stop_lv0
                    .set(!flacn.event.is_emergency_stop_lv0.get());
            };
        });

        let order = thread_variable!("key", "lidar");

        let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
            mpsc::channel();

        let mut thread: HashMap<&str, fn(Sender<String>, SenderOrders)> =
            std::collections::HashMap::new();

        thread.insert("key", |panic_msg: Sender<String>, msg: SenderOrders| {
            Rthd::<String>::send_panic_msg(panic_msg);
            loop {
                let order = Mode::input_key();
                msg.send(order).unwrap();
            }
        });

        Rthd::<String>::thread_generate(thread, &sendr_err_handles, &order);

        loop {
            match order.get("lidar").unwrap().1.try_recv() {
                Ok(e) => {
                    flag_controler.event.order.set(e);
                    flag_controler.load_fnc("set_emergency_stop");
                }
                Err(_) => {}
            };

            match order.get("key").unwrap().1.try_recv() {
                Ok(e) => {
                    if e == config::BREAK {
                        break;
                    } else {
                        flag_controler.event.order.set(e);
                        flag_controler.load_fnc("set_emergency_stop");
                        flag_controler.load_fnc("emergency_stop");
                        flag_controler.load_fnc("move");
                        flag_controler.load_fnc("is_stop");
                        flag_controler.load_fnc("is_emergency_stop");
                    }
                }
                Err(_) => {}
            };

            terminal
                .draw(|f| {
                    tui::key_ui(f, &flag_controler);
                })
                .unwrap();
            //flag_controler.load_fnc("debug");

            time_sleep(0, 60);
        }

        //tui::end();
    }

    /// キー入力
    fn input_key() -> u32 {
        let key = getch::Getch::new();
        let key_order_u8 = key.getch().unwrap();
        //println!("{}", key_order_u8);

        let order = match key_order_u8 {
            119 => {
                // w
                0x1FEEFFFF
            }
            97 => {
                // a
                0x1F7EFFFF
            }
            115 => {
                // s
                0x1F77FFFF
            }
            100 => {
                // d
                0x1FE7FFFF
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

            _ => config::None,
        };
        order
    }

    fn test_order_compare() {}
}
