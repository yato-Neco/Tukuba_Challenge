use std::collections::HashMap;
use std::io::Stdout;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use super::key::input_key;
use crate::robot::setting::Settings;
use crate::robot::{self, config};
use crate::thread_variable;
use ::tui::backend::CrosstermBackend;
use ::tui::Terminal;
use config::SenderOrders;
use flacon::{Event, FlaCon, Flags};
use getch;
use gps::{self, GPS};
use lidar::ydlidarx2;
use mytools::time_sleep;
use robot::tui;
use robot_gpio::Moter;
use robot_serialport::RasPico;
use rthred::{send, sendG, Rthd, RthdG};
use scheduler::Scheduler;
use wt901_rs::WT901;

struct SRP<'a> {
    opcode: u32,
    bytes: &'a [u8],
}

#[derive(Debug, Clone)]
pub struct AutoEvents {
    pub is_core_stop: bool,
    pub is_lidar_stop: bool,
    pub is_gps_fix_stop: bool,
    pub is_debug: bool,
    pub is_break: bool,
    pub is_move: bool,
    pub is_trune: bool,
    pub is_first_time: bool,
    pub is_continue: bool,
    pub is_interruption: bool,
    pub opcode: u32,
    pub opcode_history: Vec<u32>,
    pub latlot: (f64, f64),
    pub trun_azimuth: f64,
    pub azimuth: f64,
    pub maneuver: &'static str,
}

struct AutoModule {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub moter_controler: Moter,
    pub gps: GPS,
    pub scheduler: Scheduler,
}

pub fn auto() {
    let terminal = tui::start();
    let setting_file = Settings::load_setting("./settings.yaml");
    let mut scheduler = Scheduler::start();

    let gps_setting = setting_file.load_gps_serial();
    let nav_setting = setting_file.load_waypoint();
    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();
    let moter_controler = Moter::new(right_moter_pin, left_moter_pin);
    let mut gps = GPS::new(true);
    gps.waypoints = nav_setting;

    //モジュールをflag内で扱うための構造体
    let module = AutoModule {
        terminal,
        moter_controler,
        gps,
        scheduler,
    };

    let event = AutoEvents {
        is_core_stop: false,
        is_break: false,
        is_continue: false,
        is_debug: false,
        is_gps_fix_stop: false,
        is_lidar_stop: false,
        is_move: false,
        is_trune: false,
        is_first_time: true,
        is_interruption: false,
        opcode: 0xfffffff,
        opcode_history: Vec::new(),
        latlot: (0.0, 0.0),
        trun_azimuth: 0.0,
        azimuth: 0.0,
        maneuver: "Start",
    };

    let mut flag_controler = FlaCon::new(module, event);

    flag_controler.add_fnc("moter_control", |flacn| {
        let opcode = flacn.event.opcode;

        if flacn.event.opcode == config::EMERGENCY_STOP {
            flacn.module.moter_controler.moter_control(opcode);
            flacn.event.is_core_stop = !flacn.event.is_core_stop;
        };

        if opcode != config::NONE
            && !flacn.event.is_core_stop
            && !flacn.event.is_gps_fix_stop
            && flacn.event.is_lidar_stop
        {
            flacn.module.moter_controler.moter_control(opcode);
        }
    });

    flag_controler.add_fnc("gps_nav", |flacn| {
        flacn.event.is_break = !flacn.module.gps.nav();
    });

    flag_controler.add_fnc("gps_Fix", |flacn| {
        if flacn.module.gps.is_fix.unwrap_or(false) {
            match flacn.module.gps.nowpotion {
                Some(latlot) => {
                    flacn.module.gps.nowpotion_history.push(latlot);
                }
                None => {}
            };
        } else {
        }
    });

    flag_controler.add_fnc("first_time", |flacn| {
        //is_first_time
        if flacn.event.is_first_time {
            flacn.event.maneuver = "first_time wait fix";

            // gps 通信中 と is_first_time が true
            if flacn.module.gps.is_fix.unwrap_or(false) {
                flacn.module.gps.is_nowpotion_history_sub =
                    flacn.module.gps.nowpotion_history_sub();
                flacn.event.maneuver = "nowpotion_history_sub";

                flacn.event.opcode = config::FRONT;
                flacn.load_fnc("moter_control"); //モーター動かす。

                // gps 通信中 と is_first_time と角度を求める関数 が true
                if flacn.module.gps.is_nowpotion_history_sub {
                    flacn.event.maneuver = "frist_calculate_azimuth";

                    flacn.module.gps.now_azimuth = Some(flacn.module.gps.frist_calculate_azimuth());

                    flacn.event.opcode = config::STOP;
                    flacn.load_fnc("moter_control"); //モーター動かす。

                    flacn.event.is_continue = false;
                }
            }
        }
    });

    flag_controler.add_fnc("in_waypoint", |flacn| {
        if flacn.module.gps.in_waypoint && !flacn.event.is_first_time {
            flacn.event.maneuver = "in_waypoint";

            /*
            flacn.event.trun_azimuth =
                flacn.module.gps.azimuth - flacn.module.gps.now_azimuth.unwrap();
            */
            

            flacn.event.opcode = config::STOP;
            flacn.load_fnc("moter_control");
            time_sleep(1, 0);

            flacn.load_fnc("rotate");

            flacn.event.opcode = config::STOP;
            flacn.load_fnc("moter_control");
            time_sleep(1, 0);

            flacn.event.opcode = config::FRONT;
            flacn.load_fnc("moter_control");
        }
    });

    flag_controler.add_fnc("rotate", |flacn| {
        flacn.event.opcode = config::STOP;
        flacn.load_fnc("moter_control");
        time_sleep(0, 50);


        let azimuth = flacn.module.gps.now_azimuth;

        flacn.module.gps.wt901.ang.unwrap_or((0.0,0.0,0.0)).0;




        //flacn.event.opcode = 0x1F4AFFFF;
        //flacn.event.opcode = 0x1FA4FFFF;

        
        //while すべき
        loop {

            flacn.event.opcode = 0x1F4AFFFF;
            flacn.load_fnc("moter_control");
            break;
            time_sleep(0, 10)
        }
    });

    let opcode = thread_variable!("key", "lidar");
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
            time_sleep(0, 10);
        }
    });

    /*
    flag_controler.add_fnc("rotate", |flacn| {
        //let mut stoptime: i128 = 0;

        if flacn.event.is_lidar_stop {
            flacn.event.is_interruption = !flacn.event.is_interruption;

            if flacn.event.is_interruption {
                flacn.module.scheduler.add_time_counter();
            } else {
                flacn.module.scheduler.end();
            }
            println!("{}", flacn.module.scheduler.nowtime());
        }
    });
    */
    

    let (gps_sender, gps_receiver) = std::sync::mpsc::channel::<String>();
    let (wt901_sender, wt901_receiver) = std::sync::mpsc::channel::<WT901>();

    /*
    RthdG::<(), Settings>::_thread_generate(
        "gps",
        &sendr_err_handles,
        setting_file,
        |panic_msg, setting| {
            Rthd::<String>::send_panic_msg(panic_msg);
            let lidar_setting  =  (setting.load_lidar().0 , setting.load_lidar().1, 2000_usize);
            //module_loop(setting.load_gps_serial(), ("".to_owned(),9600,2000), lidar_setting , gps_msg, lidar_msg, wt901_msg);
        },
    );
    */

    Rthd::<String>::thread_generate(thread, &sendr_err_handles, &opcode);

    flag_controler.module.gps.generate_rome();
    loop {
        match gps_receiver.try_recv() {
            Ok(e) => {
                flag_controler.module.gps.original_nowpotion = e.clone();
                flag_controler.module.gps.parser(e);
            }
            Err(_) => {}
        }

        match wt901_receiver.try_recv() {
            Ok(e) => {
                flag_controler
                    .module
                    .gps
                    .rome
                    .set_azimuth(e.ang.unwrap().0 as f64);
            }
            Err(_) => {}
        }

        // nowpotsiton_history push
        flag_controler.load_fnc("gps_Fix");
        flag_controler.load_fnc("first_time");

        // Key
        match opcode.get("key").unwrap().1.try_recv() {
            Ok(e) => {
                if e == config::BREAK {
                    flag_controler.event.opcode = config::EMERGENCY_STOP;
                    flag_controler.load_fnc("emergency_stop");

                    flag_controler.event.maneuver = "exit";
                    tui::end(&mut flag_controler.module.terminal);
                    break;
                } else if e == config::EMERGENCY_STOP {
                    flag_controler.event.opcode = e;
                    flag_controler.load_fnc("emergency_stop");
                }
            }
            Err(_) => {}
        };

        if flag_controler.event.is_continue {
            time_sleep(0, 1);
            continue;
        }

        //flag_controler.load_fnc("rotate");
        flag_controler.load_fnc("in_waypoint");
        flag_controler.load_fnc("gps_nav");

        time_sleep(0, 10);
    }
}

fn module_loop(
    gps_setting: (String, u32, usize),
    wt901_setting: (String, u32, usize),
    lidar_setting: (String, u32, usize),
    gps_msg: Sender<String>,
    lidar_msg: Sender<String>,
    wt901_msg: Sender<WT901>,
) {
    let mut gps_port = match serialport::new(gps_setting.0, gps_setting.1)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => p,
        Err(_) => panic!(),
    };

    let mut wt901_port = match serialport::new(wt901_setting.0, wt901_setting.1)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => p,
        Err(_) => panic!(),
    };

    let mut lidar_port = match serialport::new(lidar_setting.0, lidar_setting.1)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => p,
        Err(_) => panic!(),
    };

    let mut gps_serial_buf: Vec<u8> = vec![0; gps_setting.2];
    let mut wt901_serial_buf: Vec<u8> = vec![0; wt901_setting.2];
    let mut lidar_serial_buf: Vec<u8> = vec![0; lidar_setting.2];
    let mut wt901 = WT901::new();

    loop {
        match gps_port.read(gps_serial_buf.as_mut_slice()) {
            Ok(t) => {
                //serial_buf[..t].to_vec();
                let gps_data = String::from_utf8_lossy(&gps_serial_buf[..t]).to_string();
                sendG(gps_data, &gps_msg);
                //msg.send(gps_data).unwrap();
            }
            Err(_) => {}
        }

        match wt901_port.read(wt901_serial_buf.as_mut_slice()) {
            Ok(t) => {
                let data = wt901_serial_buf[..t].to_vec();

                //cope_serial_data(data);
                wt901.cope_serial_data(data);

                sendG(wt901.clone(), &wt901_msg);
            }

            Err(_) => {}
        }

        match lidar_port.read(lidar_serial_buf.as_mut_slice()) {
            Ok(t) => {
                let mut data = lidar_serial_buf[..t].to_vec();
                let points = ydlidarx2(&mut data);
            }
            Err(_) => {}
        }
    }
}
