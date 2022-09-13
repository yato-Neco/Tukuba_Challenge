use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex};
use std::{panic, thread};

mod order;
mod robot;
mod rthred;
mod sensor;
mod mode;
mod xtools;
use rthred::{send, Rthd};
use sensor::gps::GPSmodule;
use xtools::{ms_sleep, time_sleep, warning_msg};

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
        "display" => {},
        "k" => key(),
        "m" => manual(),
        "a" => auto(),
        "d" => {},
        _ => {}
    }
}

fn display() {

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
            analysis(order);
        }
    }
}

fn key() {
    loop {
        let a = getch::Getch::new();
        let b = a.getch().unwrap();
        //println!("{}", b);

        let order = match b {
            119 => {
                "w";
                0x1FAAFFFF
            }
            97 => {
                "a";
                0x1F29FFFF
            }
            100 => {
                "d";
                0x1F92FFFF
            }
            115 => {
                "s";
                0x1F22FFFF
            }
            32 => {
                "stop";
                order::STOP
            }
            3 => {
                "break";
                order::BREAK
            }

            _ => 0xFFFFFFF,
        };

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
            analysis(order);
        }
    }
}

fn auto() {
    let mut threads: HashMap<&str, fn(Sender<String>, SenderOrders)> = HashMap::new();

    threads.insert("gps", gps);
    threads.insert("lidar", lidar);

    let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    let (sendr_msg, receiver_msg): (SenderOrders, ReceiverOrders) = mpsc::channel();

    // TODO: 各スレッドに命令を飛ばす。 <- 無理です
    let (sendr_msg1, receiver_msg1): (SenderOrders, ReceiverOrders) = mpsc::channel();

    Rthd::thread_generate(threads, &sendr_err_handles, &sendr_msg);     

    let mut pause = false;

    loop {
        let result = match receiver_msg.try_recv() {
            Ok(e) => Some(e),
            Err(_) => None,
        };

        /*
        match result  {
            Some(e) => {

            }
            None => {}
        }
        */

        if result != None {
            let order: u32 = result.unwrap();
            if ((order & 0xF0000000) >> 28_u8) == 0 {
                //println!("特権コードー");
                let privileged_instruction: u8 = ((order & 0x0000000F) >> 0) as u8;
                println!("特権コードー　{}",privileged_instruction);

                match privileged_instruction {
                    1 => {
                        pause = !pause;
                        
                        continue;
                    },
                    3 => break,
                    4 => break,
                    14 => panic!("特権命令、パニック‼"),
                    _ => {}
                }
            } else {
                if pause { 
                    continue;
                }
                analysis(order);
            }
            /*
            println!(
                "right {} : left {}",
                (d & 0x00F00000) >> 20,
                (d & 0x000F0000) >> 16
            );
            */
        }

    }
   
}

#[test]
fn test() {

}

fn analysis(order: u32) {
    let rM: u8 = ((order & 0x00F00000) >> 20) as u8;
    let lM: u8 = ((order & 0x000F0000) >> 16) as u8;
    //println!("{} {}",rM,lM);

    match (rM, lM) {
        (1..=7, 1..=7) => {
            println!("後進 {} {}", -1 * rM as i8, -1 * lM as i8);
        }
        (8..=14, 8..=14) => {
            println!("前進 {} {}", rM - 7, lM - 7);
        }
        (0, 0) => {
            println!("ストップ");
        }
        (1..=7, 8..=14) => {
            println!("回転 {} {}", -1 * rM as i8, lM - 7);
        }
        (8..=14, 1..=7) => {
            println!("回転 {} {}", rM - 7, -1 * lM as i8);
        }
        _ => {
            //println!("その他 {} {}", rM, lM);
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
    Rthd::send_panic_msg(panic_msg);
    time_sleep(0, 5);
    msg.send(0x0FFFFFF1).unwrap();
    //time_sleep(0, 1);
    //msg.send(0x0FFFFFF1).unwrap();

}

fn gps(panic_msg: Sender<String>, msg: Sender<u32>) {

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

    tmp.load_waypoint();

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
                //println!("{:x}", order);

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

        msg.send(0x0000).unwrap();
    }
}


#[test]
fn motor_rotate() {
    let r = 105.0;
    let cir = ((r / 2.0) * (std::f64::consts::PI * 2.0)) / 4.0;

    println!("{}", cir);
}
