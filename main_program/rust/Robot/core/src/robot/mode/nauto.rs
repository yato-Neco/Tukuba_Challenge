use std::{
    collections::HashMap,
    io::Stdout,
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};

use flacon::{Event, FlaCon, Flags};
use gps::gps::Nav;
use mytools::{ms_sleep, time_sleep};
use robot_gpio::Moter;
use rthred::{send, sendG};
use tui::{backend::CrosstermBackend, Terminal};
use wt901::WT901;

use crate::{
    robot::{
        config::{self, SenderOrders},
        setting::Settings,
    },
    thread_variable,
};

use super::test2::Scheduler;

#[derive(Debug, Clone)]
pub struct AutoEvents {
    pub is_core_stop: bool,
    pub is_debug: bool,
    pub is_break: bool,
    pub is_move: bool,
    pub is_trune: bool,
    pub is_first_time: bool,
    pub is_continue: bool,
    pub is_flash: bool,
    pub trne_threshold: f64,
    pub opcode: u32,
    pub maneuver: &'static str,
}

struct AutoModule {
    //pub terminal: Terminal<CrosstermBackend<Stdout>>,
    //pub moter_controler: Moter,
    pub nav: Nav,
    pub scheduler: Scheduler,
    pub moter_controler: Moter,
    pub wt901: WT901,
}

pub fn nauto() {
    let setting_file = Settings::load_setting("./settings.yaml");
    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();
    let moter_controler = Moter::new(right_moter_pin, left_moter_pin);
    let gps_setting = setting_file.load_gps_serial();
    let lidar_setting = setting_file.load_lidar();
    let wt901 = WT901::new();

    let module = AutoModule {
        nav: Nav::init(),
        scheduler: Scheduler::start(),
        moter_controler: moter_controler,
        wt901: wt901,
    };

    let event = AutoEvents {
        is_core_stop: false,
        is_break: false,
        is_continue: true,
        is_debug: false,
        is_move: false,
        is_trune: false,
        is_first_time: true,
        is_flash: true,
        trne_threshold: 3.5,
        opcode: 0xfffffff,

        maneuver: "Start",
    };

    let mut flacn = FlaCon::new(module, event);

    //let opcode = thread_variable!("operator");

    let thread: HashMap<&str, fn(Sender<String>, SenderOrders)> = std::collections::HashMap::new();

    let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    //thread.insert("operator", operator);

    let mut gps_port = match serialport::new(gps_setting.0, gps_setting.1)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => Some(p),
        Err(_) => {
            mytools::warning_msg("None GPS Module");
            None
        }
    };

    let mut wt901_port = match serialport::new("COM3", 9600)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => Some(p),
        Err(_) => {
            mytools::warning_msg("None wt901 Module");
            None
        }
    };

    let mut lidar_port = match serialport::new(lidar_setting.0, lidar_setting.1)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => Some(p),
        Err(_) => {
            mytools::warning_msg("None LiDAR Module");
            None
        }
    };

    let mut gps_serial_buf: Vec<u8> = vec![0; gps_setting.2];
    let mut wt901_serial_buf: Vec<u8> = vec![0; 2000];
    let mut lidar_serial_buf: Vec<u8> = vec![0; 2000];

    //waypoint設定 -->
    let mut waypoints: Vec<(f64, f64)> = Vec::new();
    waypoints.push((36.064227, 136.221376));
    //<--

    // demo -->
    //flacn.module.nav.lat_lon = Some((36.064226, 136.221375));
    flacn.module.nav.set_lat_lot((36.064226, 136.221375));
    flacn.module.nav.robot_move(0.0, 0.0);
    flacn.module.nav.set_lat_lot((36.064226, 136.221376));
    flacn.module.nav.robot_move(0.0, 0.0);
    //println!("{:?}", flacn.module.nav);
    //flacn.event.is_first_time = false;
    //<--

    flacn.add_fnc("first_time", |flacn| {
        flacn.event.maneuver = "first_time";
        //(flacn.module.send)(config::FRONT, &flacn.module.msg);
        println!("{}", flacn.module.nav.lat_lon_history.len());
        flacn.module.moter_controler.moter_control(config::FRONT);

        if flacn.module.nav.lat_lon_history.len() > 1 {
            flacn.module.nav.set_start_index();
            flacn.module.nav.frist_calculate_azimuth();

            //(flacn.module.send)(config::STOP, &flacn.module.msg);
            flacn.module.moter_controler.moter_control(config::STOP);

            flacn.event.is_continue = false;
        }
    });

    flacn.add_fnc("rote", |flacn| {
        //println!("{}",flacn.module.wt901.aziment.2);

        let azimuth = flacn.module.nav.start_azimuth + flacn.module.nav.next_azimuth;

        let trne_threshold_azimuth = (
            flacn.module.wt901.aziment.2 - flacn.event.trne_threshold,
            flacn.module.wt901.aziment.2 + flacn.event.trne_threshold,
        );
        //println!("{}",azimuth);

        if azimuth > 0.0 {
            flacn.module.moter_controler.moter_control(0x1F5CFFFF);
        }else{
            flacn.module.moter_controler.moter_control(0x1FC5FFFF);
        }

        if trne_threshold_azimuth.0 <= azimuth && azimuth >= trne_threshold_azimuth.1 {
            println!("Ok");

            flacn.module.moter_controler.moter_control(config::STOP);
            ms_sleep(100);
            flacn.module.moter_controler.moter_control(config::FRONT);
            flacn.event.is_trune = false;
        }
    });

    flacn.module.nav.add_waypoints(waypoints);

    loop {
        match gps_port {
            Some(ref mut gps) => match gps.read(gps_serial_buf.as_mut_slice()) {
                Ok(t) => {
                    let gps_data = String::from_utf8_lossy(&gps_serial_buf[..t]).to_string();
                    flacn.module.nav.gps_senser.parser(gps_data);
                    if flacn.module.nav.gps_senser.is_fix {
                        flacn
                            .module
                            .nav
                            .set_lat_lot(flacn.module.nav.gps_senser.lat_lon.unwrap());
                    }
                    flacn.module.nav.robot_move(0.0, 0.0);
                }
                Err(_) => {
                    flacn.module.nav.gps_senser.is_fix = false;
                }
            },
            None => {
                //flacn.module.nav.set_lat_lot((36.164227, 136.241375));
                //mytools::warning_msg("non");

                if true {
                    flacn.module.nav.gps_senser.is_fix = true;
                    //flacn.event.is_continue = false;
                } else {
                    flacn.module.nav.gps_senser.is_fix = false;
                }
            }
        }

        match wt901_port {
            Some(ref mut wt901) => match wt901.read(wt901_serial_buf.as_mut_slice()) {
                Ok(t) => {
                    let data = wt901_serial_buf[..t].to_vec();
                    flacn.module.wt901.cope_serial_data(data);
                    flacn.module.wt901.z_aziment();
                    //println!("{}",flacn.module.wt901.aziment.2);
                }

                Err(_) => {}
            },
            None => {}
        }

        match lidar_port {
            Some(ref mut lidar) => match lidar.read(lidar_serial_buf.as_mut_slice()) {
                Ok(t) => {
                    let data = lidar_serial_buf[..t].to_vec();
                }

                Err(_) => {}
            },
            None => {}
        }

        flacn.module.nav.robot_move(0.0, 0.0);


        flacn.load_fnc_is("rote", flacn.event.is_trune);

        if flacn.event.is_trune {
            time_sleep(0, 10);
            continue;
        }
        // ↓ ターン中は実行されない。

        flacn.load_fnc_is(
            "first_time",
            flacn.module.nav.gps_senser.is_fix && flacn.event.is_first_time,
        );

        if flacn.event.is_first_time && flacn.event.is_continue {
            time_sleep(0, 10);
            println!("continue");
            continue;
        }
        // ↓ 最初の処理が終わらないと処理されない。

        let mut flag = flacn.module.nav.in_waypoint(); 
        //println!("{:?}", flag);

        // 最終地点
        if flag.1 {
            flacn.module.moter_controler.moter_control(config::STOP);
            ms_sleep(800);
            flacn.module.moter_controler.pwm_all_clean();
            ms_sleep(800);
            break;
        }

        // waypoint 処理
        if flag.0 || flacn.event.is_first_time {
            flacn.event.is_first_time = false;
            flacn.module.nav.waypoint_azimuth_distance();
            flacn.module.wt901.aziment.2 = 0.0;
            println!("trune");

            println!(
                "start_azimuth {} next_azimuth: {} : {}",
                flacn.module.nav.start_azimuth,
                flacn.module.nav.next_azimuth,
                flacn.module.nav.start_azimuth + flacn.module.nav.next_azimuth
            );
            flacn.event.is_trune = true;
            flag.0 = false;
        }

        //time_sleep(0, 10);
        ms_sleep(10);
        //flacn.module.nav.set_lat_lot((36.064227, 136.221376));

    }
}


/*
fn operator(panic_msg: Sender<String>, msg: SenderOrders) {
    let setting_file = Settings::load_setting("./settings.yaml");
    let gps_setting = setting_file.load_gps_serial();

    let module = AutoModule {
        nav: Nav::init(),
        scheduler: Scheduler::start(),
        send: send,
        msg: msg,
    };

    let event = AutoEvents {
        is_core_stop: false,
        is_break: false,
        is_continue: false,
        is_debug: false,
        is_move: false,
        is_trune: false,
        is_first_time: true,
        opcode: 0xfffffff,
        azimuth: 0.0,
        maneuver: "Start",
    };

    let mut flacn = FlaCon::new(module, event);

    let mut gps_port = match serialport::new(gps_setting.0, gps_setting.1)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => p,
        Err(_) => panic!(),
    };

    let mut gps_serial_buf: Vec<u8> = vec![0; gps_setting.2];

    //waypoint設定 -->
    let mut waypoints = Vec::new();
    waypoints.push((36.064225, 136.221375));
    flacn.module.nav.add_waypoints(waypoints);
    //<--

    flacn.add_fnc("first_time", |flacn| {
        flacn.event.maneuver = "first_time";
        (flacn.module.send)(config::FRONT, &flacn.module.msg);

        if flacn.module.nav.lat_lon_history.len() > 1 {
            flacn.event.azimuth = flacn.module.nav.frist_calculate_azimuth();

            (flacn.module.send)(config::STOP, &flacn.module.msg);

            flacn.event.is_first_time = false;
        }
    });

    loop {
        match gps_port.read(gps_serial_buf.as_mut_slice()) {
            Ok(t) => {
                let gps_data = String::from_utf8_lossy(&gps_serial_buf[..t]).to_string();
                flacn.module.nav.gps_senser.parser(gps_data);
                if flacn.module.nav.gps_senser.is_fix {
                    flacn
                        .module
                        .nav
                        .set_lat_lot(flacn.module.nav.gps_senser.lat_lon.unwrap());
                }
            }
            Err(_) => {}
        }

        flacn.load_fnc_is(
            "first_time",
            flacn.module.nav.gps_senser.is_fix && flacn.event.is_first_time,
        );
        flacn.module.nav.robot_move(0.0, 0.0);

        if !flacn.event.is_first_time {
            continue;
        }

        if flacn.module.nav.is_in_waypoint() {
            break;
        }

        //send(config::FRONT, &msg);
    }
}

fn serial() {}
*/
