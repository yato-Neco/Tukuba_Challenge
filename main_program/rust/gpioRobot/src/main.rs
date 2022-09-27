use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex};
use std::{panic, thread};

mod mode;
mod order;
mod robot;
mod rthred;
mod sensor;
mod xtools;
use robot::{moter::MoterGPIO,settings::Settings};
use rthred::{send, Rthd};
use sensor::gps::GPSmodule;
use xtools::{roundf, time_sleep, warning_msg};

use clap::Parser;
use getch;
use yaml_rust::Yaml;


type SenderOrders = Sender<u32>;
type ReceiverOrders = Receiver<u32>;



#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    mode: String,
}

fn main() {
    let args = Args::parse();

    let settings_path: &str = "./settings.yaml";
    const S: &str = r#"
     _____ _             _     _____       _           _   
    / ____| |           | |   |  __ \     | |         | |  
   | (___ | |_ __ _ _ __| |_  | |__) |___ | |__   ___ | |_ 
    \___ \| __/ _` | '__| __| |  _  // _ \| '_ \ / _ \| __|
    ____) | || (_| | |  | |_  | | \ \ (_) | |_) | (_) | |_ 
   |_____/ \__\__,_|_|   \__| |_|  \_\___/|_.__/ \___/ \__|
    "#;

    //println!("{}", S);

    match args.mode.as_str() {
        "manual" => manual(),
        "auto" => auto(settings_path),
        "key" => key(settings_path),
        "display" => {}
        "k" => key(settings_path),
        "m" => manual(),
        "a" => auto(settings_path),
        "d" => {}
        _ => {}
    }
}

fn display(settings_path: &str) {}

fn manual() {
    let mut moter = MoterGPIO::new([25, 24], [22, 23]);

    println!("0x{};", "-".repeat(2 << 2));
    loop {
        let mut order = String::new();
        std::io::stdin().read_line(&mut order).ok();
        order = order.trim().to_string();

        if order.len() <= 2 {
            continue;
        };

        let (front, back) = order.split_at(2);

        //
        if front != "0x" {
            continue;
        };
        //

        let order: u32 = match u32::from_str_radix(&back, 16) {
            Ok(e) => e,
            Err(_) => continue,
        };

        println!("0x{};", "-".repeat(2 << 2));

        if ((order & 0xF0000000) >> 28_u8) == 0 {
            //println!("特権コードー");
            let privileged_instruction: u8 = ((order & 0x0000000F) >> 0) as u8;
            //println!("{}",privileged_instruction);

            match privileged_instruction {
                1 => panic!("特権命令、パニック‼"),
                3 => break,
                4 => break,
                _ => {}
            }
        } else {
            MoterGPIO::moter_control(order, &mut moter);
        }
    }
}

fn key(settings_path: &str) {
    let settings_yaml = Settings::load_setting(settings_path);
    let mut settings_spped_orders = [0_u32; 4];

    let moter_pin = Settings::load_moter_pin(&settings_yaml);

    let mut moter = MoterGPIO::new(moter_pin.0, moter_pin.1);

    for i in settings_yaml["Robot"]["Key_mode"]["speed"]
        .clone()
        .into_iter()
        .enumerate()
    {
        settings_spped_orders[i.0] = i.1.as_i64().unwrap() as u32;
    }

    drop(settings_yaml);

    let mut pause = false;

    loop {
        let key = getch::Getch::new();
        let key_order_u8 = key.getch().unwrap();
        //println!("{}", b);

        let order = match key_order_u8 {
            119 => {
                // w
                settings_spped_orders[0]
            }
            97 => {
                // a
                settings_spped_orders[1]
            }
            115 => {
                // s
                settings_spped_orders[2]
            }
            100 => {
                // d
                settings_spped_orders[3]
            }
            32 => {
                // stop
                order::STOP
            }
            3 => {
                // break
                order::BREAK
            }

            _ => 0xFFFFFFF,
        };

        if ((order & 0xF0000000) >> 28_u8) == 0 {
            let privileged_instruction: u8 = ((order & 0x0000000F) >> 0) as u8;

            match privileged_instruction {
                1 => {
                    pause = !pause;

                    continue;
                }
                3 => break,
                4 => break,
                _ => {}
            }
        } else {
            if pause {
                continue;
            }
            MoterGPIO::moter_control(order, &mut moter);
        }
    }
}

fn auto(settings_path: &str) {
    let mut threads: HashMap<&str, fn(Sender<String>, SenderOrders, Yaml)> = HashMap::new();

    let settings_yaml = Settings::load_setting(settings_path);

    threads.insert("gps", gps);
    threads.insert("lidar", lidar);

    let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    let (sendr_msg, receiver_msg): (SenderOrders, ReceiverOrders) = mpsc::channel();

    Rthd::thread_generate(threads, &sendr_err_handles, &sendr_msg, settings_yaml);

    let mut pause = false;

    loop {
        let result = match receiver_msg.try_recv() {
            Ok(e) => Some(e),
            Err(_) => None,
        };

        if result != None {
            let order: u32 = result.unwrap();
            if ((order & 0xF0000000) >> 28_u8) == 0 {
                let privileged_instruction: u8 = ((order & 0x0000000F) >> 0) as u8;
                println!("特権コードー　{}", privileged_instruction);

                match privileged_instruction {
                    1 => {
                        pause = !pause;

                        continue;
                    }
                    3 => break,
                    4 => break,
                    14 => panic!("特権命令、パニック‼"),
                    _ => {}
                }
            } else {
                if pause {
                    continue;
                }

                println!("{:?}", analysis(order));
            }
        }
    }
}

#[test]
fn test() {}

fn analysis(order: u32) -> ((f64, f64), (f64, f64)) {
    let rM: i8 = ((order & 0x00F00000) >> 20) as i8;
    let lM: i8 = ((order & 0x000F0000) >> 16) as i8;
    match (rM, lM) {
        (1..=7, 1..=7) => {
            println!("後進 {} {}", (rM - 8).abs(), (lM - 8).abs());
            (
                (0.0, roundf((rM - 8).abs() as f64 * 0.1, 10)),
                (0.0, roundf((lM - 8).abs() as f64 * 0.1, 10)),
            )
        }
        (8..=14, 8..=14) => {
            println!("前進 {} {}", rM - 4, lM - 4);
            (
                (roundf(((rM - 4) as f64 * 0.1) * 1.0, 10), 0.0),
                (roundf((lM - 4) as f64 * 0.1, 10), 0.0),
            )
        }
        (0, 0) => {
            println!("ストップ");
            ((0.0, 0.0), (0.0, 0.0))
        }
        (1..=7, 8..=14) => {
            println!("回転 {} {}", (rM - 8).abs(), lM - 4);
            (
                (0.0, roundf((rM - 8).abs() as f64 * 0.1, 10)),
                (roundf((lM - 4) as f64 * 0.1, 10), 0.0),
            )
        }
        (8..=14, 1..=7) => {
            println!("回転 {} {}", rM - 4, (lM - 8).abs());
            (
                (roundf((rM - 4) as f64 * 0.1, 10), 0.0),
                (0.0, roundf((lM - 8).abs() as f64 * 0.1, 10)),
            )
        }
        _ => {
            //println!("その他 {} {}", rM, lM);
            ((0.0, 0.0), (0.0, 0.0))
        }
    }
}

//#[cfg(target_os = "linux")]linux の場合呼び出される関数
#[cfg(target_os = "linux")]
pub fn Motor() {
    //python の importと同じ
    use robot::motor::MotorGPIO;

    //class の宣言みたいなもの
    let tmp = MotorGPIO::new([25, 24], [32, 36]);
}

//#[cfg(target_os = "windows")]はwindows の場合呼び出される関数
#[cfg(target_os = "windows")]
pub fn Motor() {
    println!("Run");
}

fn lidar(panic_msg: Sender<String>, msg: Sender<u32>, settings_yaml: Yaml) {
    Rthd::send_panic_msg(panic_msg);
    time_sleep(0, 5);
    msg.send(0x0FFFFFF1).unwrap();
    //println!("{:?}",settings_yaml["Robot"]["gps"]["waypoint"][0].as_str().unwrap());
    //time_sleep(0, 1);
    //msg.send(0x0FFFFFF1).unwrap();
}

fn gps(panic_msg: Sender<String>, msg: Sender<u32>, settings_yaml: Yaml) {
    Rthd::send_panic_msg(panic_msg);

    let mut is_beginning: bool = true;

    // nav関数の次のウェイポイントへ行くとき、なぜかズレがあるので、それを修正するフラグ管理
    // ズレがあると、回転できない。
    let mut is: bool = false;

    let mut latlot: Vec<(f64, f64)> = Vec::new();
    let mut now_azimuth = 0.0;
    let mut nlatlot: (f64, f64) = (36.000_000, 136.000_000);
    let mut order_q: [u32; 2] = [0xFFFFFFFF_u32; 2];

    println!("初期値-緯度経度 {:?}", nlatlot);

    //latlot.push((36.000_006, 136.000_003));

    let mut tmp = GPSmodule {
        r: 0.0,
        latlot: &mut latlot,
    };

    tmp.load_waypoint(
        settings_yaml["Robot"]["Gps"]["waypoint"][0]
            .as_str()
            .unwrap(),
    );

    loop {
        //(bool, (f64, f64), (f64, f64)) (false, (azimuth, distance), diff, point切り替え)
        //println!("now_point: {:?}", nlatlot);

        let flag: (bool, (f64, f64), (f64, f64), bool, &mut Vec<(f64, f64)>) = tmp.nav(nlatlot);
        //println!("{:?}",flag);
        //waypointが0になったら停止
        if flag.0 {
            send(order::BREAK, &msg);

            break;
        } else {
            let index: usize = ((flag.2 .0).abs() + (flag.2 .1).abs()) as usize;

            let order: u32 = GPSmodule::distance_con(index);
            order_q[0] = order;
            //println!("{:x}",order);

            if flag.3 {
                is = true;
                order_q = [0xFFFFFFFF_u32; 2];

                send(order::STOP, &msg);
                time_sleep(0, 1);

                continue;
            }

            if is_beginning || is {
                println!("{}", "-".repeat(20));
                println!("ウェイポイント {:?}", flag.4);
                println!("now_azi {}", now_azimuth);
                println!("azimuth: {} ", flag.1 .0);
                let azimuth = flag.1 .0;

                GPSmodule::rotate(azimuth, &mut now_azimuth, &msg);

                now_azimuth = azimuth;
                //println!("{:x}",order);
                //send(order,&msg);
                is = false;
            }
            //println!("{:x} {:x}",order_q[0], order_q[1]);
            if order_q[0] != order_q[1] {
                send(order, &msg);
            }

            //

            GPSmodule::running_simulater(&mut nlatlot, &flag.2);
        }

        time_sleep(0, 10);

        is_beginning = false;
        order_q[1] = order_q[0];
    }
}

fn s4(panic_msg: Sender<String>, msg: SenderOrders) {
    Rthd::send_panic_msg(panic_msg);

    loop {
        time_sleep(1, 0);

        msg.send(0x000000000000).unwrap();
    }
}

#[test]
fn motor_rotate() {
    let r = 105.0;
    let cir = ((r / 2.0) * (std::f64::consts::PI * 2.0)) / 4.0;

    println!("{}", cir);
}
