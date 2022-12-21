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
use robot::tui;
use crate::robot::{config, self};
use crate::robot::setting::Settings;
use crate::thread_variable;
use super::key::input_key;

use config::{
    SenderOrders
};


use std::io::Stdout;
use std::{
    cell::Cell,
    collections::HashMap,
    io::{stdout, Write},
    sync::mpsc::{self, Receiver, Sender},
};
#[derive(Debug, Clone)]
pub struct AutoEvents {
    pub is_debug: bool,
    pub is_avoidance: bool,
    pub is_break: bool,
    pub is_move: bool,
    pub is_trune: bool,
    pub is_emergency_stop_lv1: bool,
    pub is_emergency_stop_lv0: bool,
    pub order: u32,
    pub order_history: Vec<u32>,
    pub latlot: (f64, f64),
    pub first_time: bool,
    pub trun_azimuth: f64,
    pub is_continue: bool,
    pub maneuver: &'static str,
}


struct AutoModule {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub moter_controler: Moter,
    pub gps: GPS,
}



pub fn auto() {
        let terminal = tui::start();

        let setting_file = Settings::load_setting("./settings.yaml");

        //let (port, rate, buf_size) = setting_file.load_gps_serial();

        let gps_setting = setting_file.load_gps_serial();
        let nav_setting = setting_file.load_waypoint();
        let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();
        let moter_controler = Moter::new(right_moter_pin, left_moter_pin);

        let mut gps = GPS::new(false);
        gps.latlot = nav_setting;
        //モジュールをflag内で扱うための構造体
        let module = AutoModule {
            terminal,
            moter_controler,
            gps,
        };

        // Robot の event(flag)管理
        let event = AutoEvents {
            is_debug: true,
            is_avoidance: true,
            is_break: false,
            is_move: false,
            is_trune: false,
            is_emergency_stop_lv1: false,
            is_emergency_stop_lv0: false,
            order: 0xfffffff,
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
                flacn.module.moter_controler.moter_control(config::EMERGENCY_STOP);
            };
        });

        flag_controler.add_fnc("emergency_stop", |flacn| {
            // is_emergency_stop_lv0 が false で尚且つ、
            // order が前進をだったら is_move を true にする。
            flacn.load_fnc("set_emergency_stop");

            if flacn.event.is_emergency_stop_lv0 {
                flacn.module.moter_controler.moter_control(config::EMERGENCY_STOP);
            } else {
                flacn.load_fnc("set_move");
            }
        });

        flag_controler.add_fnc("set_emergency_stop", |flacn| {
            // order が EMERGENCY_STOP だったら EMERGENCY_STOP の bool を反転にする。
            if flacn.event.order == config::EMERGENCY_STOP {
                flacn.event.is_move=false;
                flacn
                    .event
                    .is_emergency_stop_lv0
                    =!flacn.event.is_emergency_stop_lv0;
            };
        });

        flag_controler.module.gps.generate_rome();

        //println!("{:?}",flag_controler.module.gps.rome);

        flag_controler.add_fnc("gps_nav", |flacn| {
            // GPS Nav 終了フラグなど
            let gps = &mut flacn.module.gps;
            let isend = gps.nav();
            //print!("{}",isend);

            flacn.event.is_break = !isend;

            // gps
        });

        flag_controler.add_fnc("gps_Fix", |flacn| {
            //flacn.module.gps.is_fix;
            // gps 受信フラグ
            //time_sleep(0, 50);

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

            //time_sleep(0, 50);
        });

        flag_controler.add_fnc("first_time", |flacn| {
            // 初期動
            // ロボット向きを求める。
            if flacn.event.first_time {


                flacn.event.maneuver = "first_time wait fix";

                let is_fix = flacn.module.gps.is_fix.unwrap_or(false);

                flacn.load_fnc("tui");
                //time_sleep(0, 50);

                if is_fix {

                    flacn.module.gps.is_nowpotion_history_sub =
                        flacn.module.gps.nowpotion_history_sub();
                    flacn.event.maneuver = "nowpotion_history_sub";

                    flacn.event.order= config::FRONT;
                    flacn.load_fnc("moter_control");
                    flacn.load_fnc("tui");
                    //time_sleep(10, 0);

                    if flacn.module.gps.is_nowpotion_history_sub {
                        flacn.event.maneuver = "frist_calculate_azimuth";
                        flacn.module.gps.now_azimuth =
                            Some(flacn.module.gps.frist_calculate_azimuth());
                            
                        flacn.event.order=config::EMERGENCY_STOP;
                        flacn.load_fnc("moter_control");

                        flacn.event.is_continue = false;
                        flacn.load_fnc("tui");
                        //time_sleep(0, 50);

                        //  in_waypoint　へ　移行す所に問題あり。in_waypointはポイント内に入らないと起動しないので first_time との間に処理の空間が生まれる。
                    }
                }
            }
            // gps
        });


        flag_controler.add_fnc("in_waypoint", |flacn| {
            if  flacn.event.first_time  && !flacn.event.is_continue {
                
                flacn.event.order = config::STOP;
                flacn.load_fnc("moter_control");
                time_sleep(2, 0);
                flacn.event.maneuver = "turn";
                //flacn.event.is_trune.set(true);
                flacn.event.order= config::TRUN;
                flacn.load_fnc("moter_control");

                time_sleep(5, 0);

                flacn.event.order= config::STOP;
                //flacn.event.is_trune.set(false);
                flacn.load_fnc("moter_control");

                time_sleep(2, 0);
                flacn.event.maneuver = "go to point";
                flacn.event.order=config::FRONT;
                flacn.load_fnc("moter_control");
                
                

            }

            // waypoint到着処理(初回は無視)
            if flacn.module.gps.in_waypoint && !flacn.event.first_time {
                flacn.event.maneuver = "in_waypoint";
                flacn.event.trun_azimuth =
                    flacn.module.gps.azimuth - flacn.module.gps.now_azimuth.unwrap();

                flacn.event.order=config::STOP;
                flacn.load_fnc("moter_control");
                //flacn.load_fnc("set_move");
                //flacn.load_fnc("is_stop");
                time_sleep(2, 0);

                // 右周り左周りを決める。
                if flacn.event.trun_azimuth > 0.0 {
                    
                } else {
                }

                flacn.event.maneuver = "turn";
                //flacn.event.is_trune.set(true);
                flacn.event.order= config::TRUN;
                flacn.load_fnc("moter_control");

                time_sleep(5, 0);

                flacn.event.order= config::STOP;
                //flacn.event.is_trune.set(false);
                flacn.load_fnc("moter_control");

                time_sleep(2, 0);
                flacn.event.maneuver = "go to point";

                //flacn.event.maneuver = "front";
                flacn.event.order=config::FRONT;
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
            let setting_file = Settings::load_setting("./settings.yaml");

            let key_bind = setting_file.load_key_bind();

            loop {
                let order = input_key(key_bind);
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

        /*
        RthdG::_thread_generate(
            "gps",
            &sendr_err_handles,
            gps_sender,
            gps_setting,
            |panic_msg, gps_sender, gps_setting| {
                Rthd::<String>::send_panic_msg(panic_msg);
                GPS::serial(&gps_setting.0, gps_setting.1, gps_setting.2, gps_sender);
            },
        );
        */
        


        Rthd::<String>::thread_generate(thread, &sendr_err_handles, &order);


        loop {
            
            // GPS
            match gps_receiver.try_recv() {
                Ok(e) => {
                    flag_controler.module.gps.original_nowpotion = e.clone();
                    flag_controler.module.gps.parser(e);
                    //println!("{:?}",flag_controler.module.gps.num_sat);

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
                        flag_controler.event.order=config::EMERGENCY_STOP;
                        flag_controler.load_fnc("emergency_stop");
                        flag_controler.event.maneuver = "exit";
                        break;
                    } else if e == config::EMERGENCY_STOP {
                        flag_controler.event.order= e;
                        flag_controler.load_fnc("emergency_stop");
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
                    flag_controler.event.order = e;
                    flag_controler.event.maneuver = "emergency_stop";
                    flag_controler.load_fnc("emergency_stop");
                    //flag_controler.load_fnc("is_emergency_stop");
                }
                Err(_) => {}
            };

            flag_controler.load_fnc("gps_nav");

            //flag_controler.load_fnc("debug");

            if flag_controler.event.is_break {
                flag_controler.event.order= config::EMERGENCY_STOP;
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
