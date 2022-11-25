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

use super::tui;
use super::{
    config::{self, SenderOrders},
    setting::Settings,
};

use std::io::Stdout;
use std::{
    cell::Cell,
    collections::HashMap,
    io::{stdout, Write},
    sync::mpsc::{self, Receiver, Sender},
};

pub struct Mode {}

pub struct AutoModule {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub moter_controler: Moter,
    pub gps: GPS,
    // pub slam: SLAM
}
pub struct RasPicoAutoModule {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub raspico_controler: RasPico,
    pub gps: GPS,
    // pub slam: SLAM
}

pub struct KeyModule {
    pub moter_controler: Moter,
}

pub struct RasPicoKeyModule {
    //pub moter_controler: Moter,
    pub raspico_controler: RasPico,
}
pub struct TestModule {
    //pub moter_controler: Moter,
    pub gps: GPS,
}

#[derive(Debug, Clone)]
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
    pub first_time: bool,
    pub trun_azimuth: f64,
    pub is_continue: bool,
    pub maneuver: &'static str,
}

/// フラグのイベント一覧
#[derive(Debug)]
pub struct KeyEvents {
    pub is_debug: bool,
    pub is_avoidance: bool,
    pub is_move: bool,
    pub is_trune:bool,
    pub is_emergency_stop_lv1: bool,
    pub is_emergency_stop_lv0: bool,
    pub is_emergency_stop_lv0_delay: bool,
    pub order: u32,
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

        let gps_setting = setting_file.load_gps_serial();

        let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);

        let mut gps = GPS::new(false);

        //モジュールをflag内で扱うための構造体
        let mut module = AutoModule {
            terminal,
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
            first_time: true,
            trun_azimuth: 0.0,
            is_continue: true,
            maneuver: "Start",
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
            if order != config::NONE {
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
                if !flacn.event.is_move.get() && order == config::NONE {
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

        //flag_controler.module.gps.latlot.push((0.001, 0.001));
        flag_controler
            .module
            .gps
            .latlot
            .push((35.627200, 139.340187));
        flag_controler
            .module
            .gps
            .latlot
            .push((35.627191, 139.341463));
        flag_controler
            .module
            .gps
            .latlot
            .push((35.627191, 139.341763));
        //flag_controler.module.gps.nowpotion = Some((0.001, 0.001));

        flag_controler.module.gps.generate_rome();

        println!("{:?}", flag_controler.module.gps.rome);

        flag_controler.add_fnc("gps_nav", |flacn| {
            // GPS Nav 終了フラグなど
            let mut gps = &mut flacn.module.gps;
            let isend = gps.nav();
            //print!("{}",isend);
            flacn.event.is_break = !isend;

            // gps
        });

        flag_controler.add_fnc("gps_Fix", |flacn| {
            //flacn.module.gps.is_fix;
            // gps 受信フラグ

            if flacn.module.gps.is_fix.unwrap_or(false) {
                match flacn.module.gps.nowpotion {
                    Some(latlot) => {
                        flacn.module.gps.nowpotion_history.push(latlot);
                    }
                    None => {}
                };
            } else {
                //flacn.event.order.set(config::EMERGENCY_STOP);
                //flacn.load_fnc("set_emergency_stop");
                //flacn.load_fnc("is_emergency_stop");
            }

            time_sleep(0, 5);
        });

        flag_controler.add_fnc("first_time", |flacn| {
            // 初期動
            // ロボット向きを求める。
            if flacn.event.first_time {
                flacn.event.maneuver = "first_time wait fix";
                let is_fix = flacn.module.gps.is_fix.unwrap_or(false);
                time_sleep(0, 5);
                if is_fix {
                    let tmp = flacn.module.gps.nowpotion_history_sub();
                    //println!("{}",tmp);
                    flacn.event.maneuver = "nowpotion_history_sub";

                    if tmp {
                        flacn.event.maneuver = "frist_calculate_azimuth";
                        flacn.module.gps.now_azimuth =
                            Some(flacn.module.gps.frist_calculate_azimuth());
                        flacn.event.is_continue = false;
                    }
                }
            }

            /*
            flacn.event.order.set(config::FRONT);
            flacn.load_fnc("move");
            flacn.load_fnc("set_move");
            flacn.load_fnc("is_stop");
            flacn.event.order.set(config::STOP);
            flacn.load_fnc("move");
            flacn.load_fnc("set_move");
            flacn.load_fnc("is_stop");
            */

            // gps
        });

        flag_controler.add_fnc("in_waypoint", |flacn| {
            // waypoint到着処理(初回は無視)
            if flacn.module.gps.in_waypoint && !flacn.event.first_time {
                flacn.event.maneuver = "in_waypoint";
                flacn.event.trun_azimuth =
                    flacn.module.gps.azimuth - flacn.module.gps.now_azimuth.unwrap();

                flacn.event.order.set(config::STOP);
                flacn.load_fnc("move");
                flacn.load_fnc("set_move");
                flacn.load_fnc("is_stop");
                time_sleep(2, 0);

                // 右周り左周りを決める。
                if flacn.event.trun_azimuth > 0.0 {
                } else {
                }

                flacn.event.maneuver = "turn";
                flacn.event.is_trune.set(true);
                flacn.event.order.set(config::TRUN);
                flacn.load_fnc("move");
                flacn.load_fnc("set_move");
                flacn.load_fnc("is_stop");
                time_sleep(5, 0);

                flacn.event.order.set(config::STOP);
                flacn.event.is_trune.set(false);
                flacn.load_fnc("move");
                flacn.load_fnc("set_move");
                flacn.load_fnc("is_stop");
                time_sleep(2, 0);

                flacn.event.maneuver = "front";
                flacn.event.order.set(config::FRONT);
                flacn.load_fnc("move");
                flacn.load_fnc("set_move");
                flacn.load_fnc("is_stop");
                flacn.event.maneuver = "go to point";
            }
        });

        flag_controler.add_fnc("not_in_waypoint", |flacn| {});

        flag_controler.add_fnc("tui", |flacn| {
            let event = flacn.event.clone();
            let module = flacn.module.gps.clone();
            flacn
                .module
                .terminal
                .draw(|f| {
                    tui::auto_ui(f, event, module);
                })
                .unwrap();
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
                time_sleep(0, 50);
            }
        });

        thread.insert("slam", |panic_msg: Sender<String>, msg: SenderOrders| {
            Rthd::<String>::send_panic_msg(panic_msg);
            loop {
                let order: u32 = 0xff;
                //send(order, msg);
                msg.send(order).unwrap();

                time_sleep(0, 50);
            }
        });

        RthdG::_thread_generate(
            "gps",
            &sendr_err_handles,
            gps_sender,
            gps_setting,
            |panic_msg, gps_sender, gps_setting| {
                Rthd::<String>::send_panic_msg(panic_msg);
                //GPS::serial(&gps_setting.0, gps_setting.1, gps_setting.2, gps_sender);
            },
        );

        Rthd::<String>::thread_generate(thread, &sendr_err_handles, &order);

        /*


        loop {
            // GPS
            match gps_receiver.try_recv() {
                Ok(e) => {
                    flag_controler.module.gps.original_nowpotion = e.clone();
                    flag_controler.module.gps.parser(e);
                    //let _ = flag_controler.module.gps.now_azimuth.unwrap() - flag_controler.module.gps.azimuth;
                }
                Err(_) => {}
            }

            flag_controler.load_fnc("tui");
            flag_controler.load_fnc("gps_Fix");
            flag_controler.load_fnc("first_time");
            flag_controler.load_fnc("in_waypoint");


            // Key
            match order.get("key").unwrap().1.try_recv() {
                Ok(e) => {
                    if e == config::BREAK {
                        flag_controler.event.order.set(config::EMERGENCY_STOP);
                        //flag_controler.event.order.set(e);
                        flag_controler.load_fnc("set_emergency_stop");
                        flag_controler.load_fnc("emergency_stop");
                        flag_controler.load_fnc("is_emergency_stop");
                        flag_controler.event.maneuver = "exit";
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

            if flag_controler.event.is_continue {
                time_sleep(0, 1);
                continue;
            }

            // GPSの信号が途切れたら。(初回は無視)
            if !flag_controler.module.gps.is_fix.unwrap_or(false)
                && !flag_controler.event.first_time
            {
                //time_sleep(0, 10);
                //continue;
            }

            // Lidar 後に SLAM
            match order.get("lidar").unwrap().1.try_recv() {
                Ok(e) => {
                    flag_controler.event.order.set(e);
                    flag_controler.event.maneuver = "emergency_stop";
                    flag_controler.load_fnc("set_emergency_stop");
                    flag_controler.load_fnc("is_emergency_stop");
                }
                Err(_) => {}
            };

            flag_controler.load_fnc("gps_nav");

            //flag_controler.load_fnc("debug");

            if flag_controler.event.is_break {
                flag_controler.event.order.set(config::EMERGENCY_STOP);
                flag_controler.load_fnc("moter_control");
                flag_controler.event.maneuver = "exit";
                break;
            }

            //let (lat,lot) = flag_controler.module.gps.nowpotion.unwrap();

            flag_controler.event.first_time = false;
            time_sleep(0, 1);
        }

        flag_controler.load_fnc("tui");

        time_sleep(2, 0);
        flag_controler.module.terminal.clear().unwrap();
        //println!("{:?}",flag_controler.module.gps.nowpotion_history);
        //tui::end();
        // */
    }

    /// test mode
    pub fn test() {
        let mut terminal = tui::start();
        let setting_file = Settings::load_setting("./settings.yaml");
        let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();
        let operation = setting_file.load_move_csv();
        let (port, rate, buf_size) = setting_file.load_gps_serial();
        let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);
        let mut gps = GPS::new(true);

        //TODO: Linuxじゃ動かない

        // Lidar も
        let module = TestModule {
            //moter_controler,
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

                let scheduler = Scheduler::start();
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
                        flacn
                            .module
                            .moter_controler
                            .moter_control(config::EMERGENCY_STOP);

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

        loop {
            let key_order = Mode::input_key();
            if key_order == config::BREAK {
                break;
            }

            if key_order == config::EMERGENCY_STOP {
                moter_sender.send((config::EMERGENCY_STOP, 0)).unwrap();
            }


            time_sleep(0, 6)
        }
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

        // mut を外したい
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
            if order != config::NONE {
                flacn.module.moter_controler.moter_control(order);
            }
        });

        flag_controler.add_fnc("move", |flacn| {
            // is_move が true だったら呼び出す。
            if flacn.event.is_move {
                println!("{:x}",flacn.event.order);
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
                println!("stop");
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

        let order = thread_variable!("key", "lidar");

        let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
            mpsc::channel();

        let mut thread: HashMap<&str, fn(Sender<String>, SenderOrders)> =
            std::collections::HashMap::new();

        thread.insert("key", |panic_msg: Sender<String>, msg: SenderOrders| {
            Rthd::<String>::send_panic_msg(panic_msg);
            loop {
                let order = Mode::input_key();
                send(order, &msg);
                time_sleep(0, 50);
            }
        });

        Rthd::<String>::thread_generate(thread, &sendr_err_handles, &order);

        loop {

            /*
            match order.get("lidar").unwrap().1.try_recv() {
                Ok(e) => {
                    flag_controler.event.order = e;
                    flag_controler.load_fnc("set_emergency_stop");
                }
                Err(_) => {}
            };
            */
            
            match order.get("key").unwrap().1.try_recv() {
                Ok(e) => {
                    if e == config::BREAK {
                        let  flag=  flag_controler.module.moter_controler.reset();
                        if flag {
                            break;   
                        }
                    } else {
                        flag_controler.event.order = e;
                        flag_controler.load_fnc("set_emergency_stop");
                        flag_controler.load_fnc("emergency_stop");
                        flag_controler.load_fnc("move");
                        flag_controler.load_fnc("is_stop");
                        flag_controler.load_fnc("is_emergency_stop");
                    }
                }
                Err(_) => {}
            };

            /*
            terminal
                .draw(|f| {
                    tui::key_ui(f, &flag_controler);
                })
                .unwrap();
            */
            

            //flag_controler.load_fnc("debug");

            time_sleep(0, 50);
        }

        //terminal.clear().unwrap();
    }

    pub fn raspico_key() {
        let mut terminal = tui::start();

        let setting_file = Settings::load_setting("./settings.yaml");

        let (port, rate) = setting_file.load_raspico();

        //let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);

        let raspico_controler = RasPico::new(&port, rate);

        let module = RasPicoKeyModule { raspico_controler };

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

        // mut を外したい
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
                flacn.module.raspico_controler.write(order);
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
                flacn.module.raspico_controler.write(config::STOP);
            };
        });
        flag_controler.add_fnc("is_emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が true の時、呼び出す
            if flacn.event.is_emergency_stop_lv0 {
                //flacn.module.raspico_controler.write(config::STOP);
            };
        });

        flag_controler.add_fnc("emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が false で尚且つ、
            // order が前進をだったら is_move を true にする。

            flacn.load_fnc("set_emergency_stop");

            if flacn.event.is_emergency_stop_lv0 && !flacn.event.is_emergency_stop_lv0_delay {
                flacn.module.raspico_controler.write(config::EMERGENCY_STOP);
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
            loop {
                let order = Mode::input_key();
                send(order, &msg);
                time_sleep(0, 50);
            }
        });

        Rthd::<String>::thread_generate(thread, &sendr_err_handles, &order);

        loop {
            /*
            match order.get("lidar").unwrap().1.try_recv() {
                Ok(e) => {
                    flag_controler.event.order = e;
                    flag_controler.load_fnc("emergency_stop");
                }
                Err(_) => {}
            };
            */
            

            match order.get("key").unwrap().1.try_recv() {
                Ok(e) => {
                    if e == config::BREAK {
                        break;
                    } else {
                        flag_controler.event.order = e;
                        flag_controler.load_fnc("emergency_stop");
                        flag_controler.load_fnc("moter_control");
                        // flag_controler.load_fnc("is_stop");
                        // flag_controler.load_fnc("is_emergency_stop");
                        flag_controler.event.is_emergency_stop_lv0_delay =
                            flag_controler.event.is_emergency_stop_lv0;
                    }
                }
                Err(_) => {}
            };

            terminal
                .draw(|f| {
                    tui::raspico_key_ui(f, &flag_controler);
                })
                .unwrap();

            //flag_controler.load_fnc("debug");

            time_sleep(0, 10);
        }

        //terminal.clear().unwrap();
        tui::end(&mut terminal);
    }

    pub fn raspico_test() {
        let mut terminal = tui::start();
        let setting_file = Settings::load_setting("./settings.yaml");
        let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();
        let operation = setting_file.load_move_csv();
        let (gps_port, gps_rate, gps_buf_size) = setting_file.load_gps_serial();
        let (rp_port, rp_rate) = setting_file.load_raspico();
        let mut raspico_controler = RasPico::new(&rp_port, rp_rate);

        let mut gps = GPS::new(true);

        //TODO: Linuxじゃ動かない

        // Lidar も
        let module = TestModule {
            //moter_controler,
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
            raspico_controler,
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
                    moter_controler: RasPico,
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
                        flacn.module.moter_controler.write(config::EMERGENCY_STOP);

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

                                    order_controler.module.moter_controler.write(e.0);

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

        loop {
            let key_order = Mode::input_key();
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

    pub fn raspico_auto() {
        let mut terminal = tui::start();

        let setting_file = Settings::load_setting("./settings.yaml");

        let (port, rate, buf_size) = setting_file.load_gps_serial();

        let gps_setting = setting_file.load_gps_serial();
        let nav_setting = setting_file.load_waypoint();
        let (rp_port, rp_rate) = setting_file.load_raspico();
        let mut raspico_controler = RasPico::new(&rp_port, rp_rate);

        let mut gps = GPS::new(true);
        gps.latlot = nav_setting;
        //モジュールをflag内で扱うための構造体
        let mut module = RasPicoAutoModule {
            terminal,
            raspico_controler,
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
            first_time: true,
            trun_azimuth: 0.0,
            is_continue: true,
            maneuver: "Start",
        };

        // mut を外したい
        let mut flag_controler = FlaCon::new(module, event);

        //flag_controler.event.is_move.set(true);

        //flag_controler.module.gps.latlot.push((0.001, 0.001));
        //flag_controler.module.gps.latlot.push((35.627200,139.340187));
        //flag_controler.module.gps.latlot.push((35.627191,139.341463));
        //flag_controler.module.gps.latlot.push((35.627191,139.341763));
        //flag_controler.module.gps.nowpotion = Some((0.001, 0.001));

        flag_controler.add_fnc("is_stop", |flacn| {
            // is_move が false だったら呼び出す。
            if !flacn.event.is_move.get() {

                //println!("{:x}",flacn.event.order.get());
            };
        });

        flag_controler.add_fnc("moter_control", |flacn| {
            let order = flacn.event.order.get();
            if order != config::NONE && !flacn.event.is_emergency_stop_lv0.get() {
                flacn.load_fnc("set_move");
                flacn.module.raspico_controler.write(order);
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
                if !flacn.event.is_move.get() && order == config::NONE {
                    flacn.event.is_move.set(false);
                } else {
                    if !flacn.event.is_emergency_stop_lv0.get() {
                        flacn.event.is_move.set(true);
                    }
                }
            }
        });

        flag_controler.add_fnc("is_stop", |flacn| {
            // is_stop が false の時、呼び出す

            if !flacn.event.is_move.get() {
                //println!("stop");
                flacn.module.raspico_controler.write(config::STOP);
            };
        });
        flag_controler.add_fnc("is_emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が true の時、呼び出す
            if flacn.event.is_emergency_stop_lv0.get() {
                flacn.module.raspico_controler.write(config::EMERGENCY_STOP);
            };
        });

        flag_controler.add_fnc("emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が false で尚且つ、
            // order が前進をだったら is_move を true にする。
            flacn.load_fnc("set_emergency_stop");

            if flacn.event.is_emergency_stop_lv0.get() {
                flacn.module.raspico_controler.write(config::EMERGENCY_STOP);
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

        flag_controler.module.gps.generate_rome();

        //println!("{:?}",flag_controler.module.gps.rome);

        flag_controler.add_fnc("gps_nav", |flacn| {
            // GPS Nav 終了フラグなど
            let mut gps = &mut flacn.module.gps;
            let isend = gps.nav();
            //print!("{}",isend);

            flacn.event.is_break = !isend;

            // gps
        });

        flag_controler.add_fnc("gps_Fix", |flacn| {
            //flacn.module.gps.is_fix;
            // gps 受信フラグ

            if flacn.module.gps.is_fix.unwrap_or(false) {
                match flacn.module.gps.nowpotion {
                    Some(latlot) => {
                        flacn.module.gps.nowpotion_history.push(latlot);
                    }
                    None => {}
                };
            } else {
                //flacn.event.order.set(config::EMERGENCY_STOP);
                //flacn.load_fnc("set_emergency_stop");
                //flacn.load_fnc("is_emergency_stop");
            }

            time_sleep(0, 5);
        });

        flag_controler.add_fnc("first_time", |flacn| {
            // 初期動
            // ロボット向きを求める。

            if flacn.event.first_time {
                flacn.load_fnc("moter_control");

                flacn.event.maneuver = "first_time wait fix";
                let is_fix = flacn.module.gps.is_fix.unwrap_or(false);
                time_sleep(0, 5);
                if is_fix {
                    flacn.module.gps.is_nowpotion_history_sub =
                        flacn.module.gps.nowpotion_history_sub();
                    //println!("{}",tmp);
                    flacn.event.maneuver = "nowpotion_history_sub";

                    if flacn.module.gps.is_nowpotion_history_sub {
                        flacn.event.maneuver = "frist_calculate_azimuth";
                        flacn.module.gps.now_azimuth =
                            Some(flacn.module.gps.frist_calculate_azimuth());
                        flacn.event.is_continue = false;
                        //  in_waypoint　へ　移行す所に問題あり。in_waypointはポイント内に入らないと起動しないので first_time との間に処理の空間が生まれる。
                    }
                }
            }
            // gps
        });


        flag_controler.add_fnc("in_waypoint", |flacn| {
            
            if  flacn.event.first_time  && !flacn.event.is_continue {
                flacn.event.order.set(config::STOP);
                flacn.load_fnc("moter_control");
                time_sleep(2, 0);
                flacn.event.maneuver = "turn";
                flacn.event.is_trune.set(true);
                flacn.event.order.set(config::TRUN);
                flacn.load_fnc("moter_control");

                time_sleep(5, 0);

                flacn.event.order.set(config::STOP);
                flacn.event.is_trune.set(false);
                flacn.load_fnc("moter_control");

                time_sleep(2, 0);
                flacn.event.maneuver = "go to point";
                flacn.event.order.set(config::FRONT);
                flacn.load_fnc("moter_control");

            }

            // waypoint到着処理(初回は無視)
            if flacn.module.gps.in_waypoint && !flacn.event.first_time {
                flacn.event.maneuver = "in_waypoint";
                flacn.event.trun_azimuth =
                    flacn.module.gps.azimuth - flacn.module.gps.now_azimuth.unwrap();

                flacn.event.order.set(config::STOP);
                flacn.load_fnc("moter_control");
                //flacn.load_fnc("set_move");
                //flacn.load_fnc("is_stop");
                time_sleep(2, 0);

                // 右周り左周りを決める。
                if flacn.event.trun_azimuth > 0.0 {
                } else {
                }

                flacn.event.maneuver = "turn";
                flacn.event.is_trune.set(true);
                flacn.event.order.set(config::TRUN);
                flacn.load_fnc("moter_control");

                time_sleep(5, 0);

                flacn.event.order.set(config::STOP);
                flacn.event.is_trune.set(false);
                flacn.load_fnc("moter_control");

                time_sleep(2, 0);
                flacn.event.maneuver = "go to point";

                //flacn.event.maneuver = "front";
                flacn.event.order.set(config::FRONT);
                flacn.load_fnc("moter_control");

            }
        });

        flag_controler.add_fnc("not_in_waypoint", |flacn| {});

        flag_controler.add_fnc("tui", |flacn| {
            let event = flacn.event.clone();
            let module = flacn.module.gps.clone();
            flacn
                .module
                .terminal
                .draw(|f| {
                    tui::auto_ui(f, event, module);
                })
                .unwrap();
        });

        flag_controler.add_fnc("tui_end", |flacon| {
            tui::end(&mut flacon.module.terminal);
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
                time_sleep(0, 50);
            }
        });

        /*
        thread.insert("slam", |panic_msg: Sender<String>, msg: SenderOrders| {
            Rthd::<String>::send_panic_msg(panic_msg);
            loop {
                //let order:u32 = 0xff;
                //send(order, msg);
                //msg.send(order).unwrap();

                time_sleep(0, 50);
            }
        });
        */

        RthdG::_thread_generate(
            "gps",
            &sendr_err_handles,
            gps_sender,
            gps_setting,
            |panic_msg, gps_sender, gps_setting| {
                Rthd::<String>::send_panic_msg(panic_msg);
                //GPS::serial(&gps_setting.0, gps_setting.1, gps_setting.2, gps_sender);
            },
        );

        Rthd::<String>::thread_generate(thread, &sendr_err_handles, &order);

        loop {
            // GPS
            match gps_receiver.try_recv() {
                Ok(e) => {
                    flag_controler.module.gps.original_nowpotion = e.clone();
                    flag_controler.module.gps.parser(e);

                    //let _ = flag_controler.module.gps.now_azimuth.unwrap() - flag_controler.module.gps.azimuth;
                }
                Err(_) => {}
            }

            flag_controler.load_fnc("tui");
            flag_controler.load_fnc("gps_Fix");
            flag_controler.load_fnc("first_time");
            flag_controler.load_fnc("in_waypoint");

            //flag_controler.load_fnc("moter_control");

            // Key
            match order.get("key").unwrap().1.try_recv() {
                Ok(e) => {
                    if e == config::BREAK {
                        flag_controler.event.order.set(config::EMERGENCY_STOP);
                        //flag_controler.event.order.set(e);
                        //flag_controler.load_fnc("set_emergency_stop");
                        flag_controler.load_fnc("emergency_stop");
                        //flag_controler.load_fnc("is_emergency_stop");
                        flag_controler.event.maneuver = "exit";
                        break;
                    } else if e == config::EMERGENCY_STOP {
                        flag_controler.event.order.set(e);
                        //flag_controler.load_fnc("set_emergency_stop");
                        flag_controler.load_fnc("emergency_stop");
                        //flag_controler.load_fnc("is_emergency_stop");
                    }
                }
                Err(_) => {}
            };

            if flag_controler.event.is_continue {
                time_sleep(0, 1);
                continue;
            }

            // GPSの信号が途切れたら。(初回は無視)
            if !flag_controler.module.gps.is_fix.unwrap_or(false)
                && !flag_controler.event.first_time
            {
                //time_sleep(0, 10);
                //continue;
            }

            // Lidar 後に SLAM
            match order.get("lidar").unwrap().1.try_recv() {
                Ok(e) => {
                    flag_controler.event.order.set(e);
                    flag_controler.event.maneuver = "emergency_stop";
                    flag_controler.load_fnc("emergency_stop");
                    //flag_controler.load_fnc("is_emergency_stop");
                }
                Err(_) => {}
            };

            flag_controler.load_fnc("gps_nav");

            //flag_controler.load_fnc("debug");

            if flag_controler.event.is_break {
                flag_controler.event.order.set(config::EMERGENCY_STOP);
                flag_controler.load_fnc("moter_control");
                flag_controler.event.maneuver = "exit";
                break;
            }

            //let (lat,lot) = flag_controler.module.gps.nowpotion.unwrap();

            flag_controler.event.first_time = false;
            time_sleep(0, 1);
        }

        flag_controler.load_fnc("tui");
        time_sleep(2, 0);
        flag_controler.load_fnc("tui_end");
        //println!("{:?}",flag_controler.module.gps.nowpotion_history);
        //tui::end();
        // */
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

            _ => config::NONE,
        };
        order
    }

    fn test_order_compare() {}
}
