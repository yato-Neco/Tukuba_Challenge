use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex};
use std::{panic, thread};

mod robot;
mod rthred;
mod sensor;
mod xtools;
use rthred::Rthd;
use sensor::gps::GPSmodule;
use xtools::{ms_sleep, time_sleep};

type SenderOrders = Sender<u32>;
type ReceiverOrders = Receiver<u32>;
use clap::Parser;
use getch;
enum Order {
    stop,
    start,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    mode: String,
    /*
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
    */
}

fn main() {
    let args = Args::parse();

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
        "auto" => auto(),
        "key" => key(),
        "k" => key(),
        "m" => manual(),
        "a" => auto(),
        _ => {}
    }
}

fn manual() {
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
    }
}

fn key() {
    loop {
        let a = getch::Getch::new();
        let b = a.getch().unwrap();
        println!("{}", b);

        match b {
            119 => {
                "w";
            }
            97 => {
                "a";
            }
            100 => {
                "d";
            }
            115 => {
                "s";
            }
            32 => {
                "stop";
            }
            3 => break,
            _ => {}
        }
    }
}

fn auto() {
    let mut threads: HashMap<&str, fn(Sender<String>, SenderOrders)> = HashMap::new();

    threads.insert("gps", gps);
    //threads.insert("lidar", lidar);

    let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    let (sendr_msg, receiver_msg): (SenderOrders, ReceiverOrders) = mpsc::channel();

    // TODO: 各スレッドに命令を飛ばす。 <- 無理です
    let (sendr_msg1, receiver_msg1): (SenderOrders, ReceiverOrders) = mpsc::channel();

    Rthd::thread_generate(threads, &sendr_err_handles, &sendr_msg);

    loop {
        let result = match receiver_msg.try_recv() {
            Ok(e) => Some(e),
            Err(_) => None,
        };

        if result != None {
            let d: u32 = result.unwrap();

            if ((d & 0xF0000000) >> 28_u8) == 0 {
                println!("特権コードー");
            }
            /*
            println!(
                "right {} : left {}",
                (d & 0x00F00000) >> 20,
                (d & 0x000F0000) >> 16
            );
            */

            analysis(d);
        }

        //ms_sleep(1500);
    }
}

#[test]
fn test() {
    // 0: 権限 0特権 以下...
    // 1:
    // 2: rigth motor speed 0 停止 F変更無し
    // 3: left motor speed ...
    // 4:
    // 5:
    // 6: 特権系命令 14 panic 1 一時停止 2 再開 3 完全停止

    let d: u32 = 0x1F994567;

    /*
    println!("{}", (d & 0xF0000000) >> 28);
    println!("{}", (d & 0x0F000000) >> 24);
    println!("{}", (d & 0x00F00000) >> 20);
    println!("{}", (d & 0x000F0000) >> 16);
    println!("{}", (d & 0x0000F000) >> 12);
    println!("{}", (d & 0x00000F00) >> 8);
    println!("{}", (d & 0x000000F0) >> 4);
    println!("{}", (d & 0x0000000F) >> 0);
    */
    let privileged_instruction: u8;

    if (d & 0xF0000000) >> 28 == 0 {
        println!("0");

        privileged_instruction = ((d & 0x0000000F) >> 0) as u8;
    } else {
        analysis(d);

        //println!("1");
    };

    //fn
}

fn analysis(order: u32) {
    let rM: u8 = ((order & 0x00F00000) >> 20) as u8;
    let lM: u8 = ((order & 0x000F0000) >> 16) as u8;
    //println!("{} {}",rM,lM);

    match (rM, lM) {
        (1..=7, 1..=7) => {
            println!("後進 {} {}", rM, lM);
        }
        (8..=14, 8..=14) => {
            println!("前進 {} {}", rM, lM);
        }
        (0, 0) => {
            println!("ストップ");
        }
        (1..=7, 8..=14) => {
            println!("回転 {} {}", rM, lM);
        }
        (8..=14, 1..=7) => {
            println!("回転 {} {}", rM, lM);
        }
        _ => {
            println!("その他 {} {}", rM, lM);
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

fn lidar(panic_msg: Sender<String>, msg: Sender<u32>) {
    time_sleep(5,0);
    msg.send(0x0F00FFFF).unwrap();
}

fn gps(panic_msg: Sender<String>, msg: Sender<u32>) {
    Rthd::send_panic_msg(panic_msg);

    let t1: f64 = 10.0_f64.powf(-6.0);

    let c2: f64 = 10.0_f64.powf(2.0);

    let mut latlot: Vec<(f64, f64)> = Vec::new();
    let mut now_azimuth = 0.0;
    let mut nlatlot: (f64, f64) = (36.000_000, 136.000_000);

    println!("初期値 {:?}", nlatlot);

    latlot.push((36.500_000, 136.000_000));
    //latlot.push((36.000000, 136.000_000));

    //latlot.push((36.500000, 136.500_000));
    //latlot.push((36.061899, 136.222481));
    //latlot.push((36.061899, 136.232481));

    let mut tmp = GPSmodule {
        r: 0.0,
        latlot: &mut latlot,
    };

    const STOP: u32 = 0x1F00FFFF;

    loop {
        //println!("{:?}", nlatlot);
        //(bool, (f64, f64), (f64, f64)) (false, (azimuth, distance), diff)
        let flag = tmp.nav(nlatlot);
        //println!("{:?}", flag);
        if flag.0 {
            msg.send(STOP).unwrap();
            //latlot.push((36.061899, 136.222483));
            break;
        } else {
            let mut azimuth = (flag.1 .0 * c2).round();
            //println!("{}", now_azimuth);
            //azimuth = azimuth + now_azimuth;

            println!("azimuth {}", azimuth / c2);
            let razi = azimuth;

            //println!("{} {} {}", azi, azi >= 0.0, azi <= 0.0);

            // 回転系
            let mut count = 0;
            loop {
                let r: bool = azimuth >= 0.0;
                let l: bool = azimuth <= 0.0;

                // シュミ系 ->

                //azi = azi + (-1.0 * azi);
                //println!("{}", razi);

                if r != l {
                    //now_azimuth = razi;

                    if count == 0 {
                        match msg.send(0x1F18FFFF) {
                            Ok(_) => {}
                            Err(_) => {}
                        };
                    }
                    if r {
                        azimuth -= 1000.0;
                    } else {
                        azimuth += 1000.0;
                    }
                }
                // <-
                //time_sleep(1);
                else if r == l {
                    //println!("回転");
                    now_azimuth = razi;

                    if count == 0 {
                        break;
                    } else {
                        match msg.send(STOP) {
                            Ok(_) => {}
                            Err(_) => {}
                        };
                        break;
                    }
                }

                count += 1;
            }
            //

            let index: usize = (flag.2 .0 + flag.2 .1).abs() as usize;

            let order: u32 = distance_con(index);

            match msg.send(order) {
                Ok(_) => {}
                Err(_) => {}
            };

            //println!("{:?}", r);

            //rintln!("distance {} {}", flag.2 .0, flag.2 .1);

            // シュミ系 ->
            println!("{:?} {:?}", nlatlot, flag.2);

            if (flag.2 .0 * t1) > 0.0 {
                nlatlot.0 += 0.1;
            } else if (flag.2 .0 * t1) < 0.0 {
                nlatlot.0 -= 0.1;
            }

            if (flag.2 .1 * t1) > 0.0 {
                nlatlot.1 += 0.1;
            } else if (flag.2 .1 * t1) < 0.0 {
                nlatlot.1 -= 0.1;
            }

            //nlatlot.0 += (flag.2 .0) * t1;
            //nlatlot.1 += (flag.2 .1) * t1;
            // <-
        }

        ms_sleep(25);
    }

    fn distance_con(index: usize) -> u32 {
        //msg.send(0xFFFF).unwrap();
        match index {
            0 => 0x1F00FFFF,
            1..=3 => 0x1F77FFFF,
            4..=6 => 0x1FAAFFFF,
            7..=9 => 0x1FCCFFFF,
            10..=12 => 0x1FDDFFFF,
            13.. => 0x1FEEFFFF,

            _ => 0x1FFFFFFF,
        }
    }

    fn rotate() {}
}

fn s4(panic_msg: Sender<String>, msg: SenderOrders) {
    Rthd::send_panic_msg(panic_msg);

    loop {
        time_sleep(1,0);

        msg.send(0x0000).unwrap();
    }
}

//#[test]
fn py_test() {
    /*unwrap()　はResult(型)で包まれた値を元の値へ戻すメゾット
    ことの時、エラー処理を追加する。
    unwrap()　だとエラーだった場合システムが止まる。

    例外系は一通りここで学べる
    https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html

    */

    sensor::tflite::python().unwrap();
}
