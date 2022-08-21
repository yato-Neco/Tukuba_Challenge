use std::collections::HashMap;
use std::panic;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, vec};

mod robot;
mod sensor;

use sensor::gps::GPSmodule;

/*
 ┌────────┐    ┌────────┐
 │thread:1│    │thread:2│
 └───┬────┘    └────┬───┘
     │              │
     └──────┬───────┘
            │
            │
 ┌──────────▼───────────┐     ┌─────────────────────────┐
 │                      │     │     send_panic_msg      │
 │  thread_generate()   │     │                         │
 │                      │     │ ┌─────────┐ ┌─────────┐ │
 └──────────┬───────────┘     │ │thread1()│ │thread2()│ ├──┐
            │                 │ └─────────┘ └─────────┘ │  │
            │                 │                         │  │
            │                 │      thread_spwan       │  │panic!
            │                 │                         │  │
            │                 └────────────▲────────────┘  │
            │                              │               │
            └──────────────────────────────┘               │
                                                           │
                                                           │
                                                        ┌──▼─┐
                                                        │main│
                                                        └────┘

*/

fn main() {
    const S: &str = r#"
     _____ _             _     _____       _           _   
    / ____| |           | |   |  __ \     | |         | |  
   | (___ | |_ __ _ _ __| |_  | |__) |___ | |__   ___ | |_ 
    \___ \| __/ _` | '__| __| |  _  // _ \| '_ \ / _ \| __|
    ____) | || (_| | |  | |_  | | \ \ (_) | |_) | (_) | |_ 
   |_____/ \__\__,_|_|   \__| |_|  \_\___/|_.__/ \___/ \__|
                                                           
                                                           
    "#;

    println!("{}", S);

    let mut threads: HashMap<&str, fn(Sender<String>, Sender<u16>)> = HashMap::new();

    threads.insert("name-s3", s3);

    let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    let (sendr_msg, receiver_msg): (Sender<u16>, Receiver<u16>) = mpsc::channel();

    thread_generate(threads, &sendr_err_handles, &sendr_msg);

    let d: u16 = 0xFFFF;

    // 0: 0 前後 1 右 2 左 3 F無視
    // 1: 16段階速度(前後) stop 0 F 無視
    // 2: ms 0: 100 1: 200 2: 300 ..
    // 3: 

    loop {
        for d in receiver_msg.try_recv() {
            println!("1: {}", (d & 0x0F00) >> 8_u8);
            println!("3: {}", (d & 0x000F) >> 0_u8);

            /*
            println!("0: {}", (d & 0xF000) >> 12);
            println!("1: {}", (d & 0x0F00) >> 8);
            println!("2: {}", (d & 0x00F0) >> 4);
            println!("3: {}", (d & 0x000F) >> 0);
            */


            //println!("catch");
        }
        ms_sleep(500);
    }
}

#[test]
fn test() {}

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

fn s3(panic_msg: Sender<String>, msg: Sender<u16>) {
    send_panic_msg(panic_msg);

    let t: f64 = 10.0_f64.powf(-6.0);

    let s:u8 = 0xff;

    let mut latlot: Vec<(f64, f64)> = Vec::new();
    let mut nlatlot = (36.000000, 136.000000);

    latlot.push((37.000000, 136.000_000));
    latlot.push((37.000000, 137.000000));
    latlot.push((36.000000, 137.000000));
    latlot.push((36.000000, 136.000000));

    //latlot.push((36.061899, 136.222481));
    //latlot.push((36.061899, 136.232481));

    let mut tmp = GPSmodule {
        r: 0.0,
        latlot: &mut latlot,
    };

    loop {
        //println!("{:?}", nlatlot);
        //(bool, (f64, f64), (f64, f64)) (false, (azimuth, distance), diff)
        let flag = tmp.nav(nlatlot);

        if flag.0 {
            msg.send(0x0000).unwrap();
            //latlot.push((36.061899, 136.222483));
            break;
        } else {
            println!("azimuth {}", flag.1 .0);
            println!("distance {}", flag.1 .1);
            //println!("now {:?}", nlatlot);
            //println!("diff {:?}", flag.2);

            //println!("{}", (flag.2 .0) + (flag.2 .1));

            match (flag.2 .0 + flag.2 .1).abs() as usize {
                0 => {
                    msg.send(0xF0FF).unwrap();
                }
                1..=3 => {
                    msg.send(0xF2FF).unwrap();
                }
                4..=6 => {
                    msg.send(0xF4FF).unwrap();
                }
                7..=9 => {
                    msg.send(0xF8FF).unwrap();
                }
                10..=12 => {
                    msg.send(0xFCFF).unwrap();
                }
                13.. => {
                    msg.send(0xFFFF).unwrap();
                }

                _ => {}
            }

            nlatlot.0 += (flag.2 .0) * t;
            nlatlot.1 += (flag.2 .1) * t;
        }

        //nlatlot.1 += t;

        ms_sleep(500);
    }

    //msg.send(0x0000).unwrap();
}

fn s4(panic_msg: Sender<String>, msg: Sender<u16>) {
    send_panic_msg(panic_msg);

    loop {
        time_sleep(1);

        msg.send(0x0000).unwrap();
    }
}

#[inline]
fn time_sleep(sec: u64) {
    thread::sleep(Duration::from_secs(sec));
}

#[inline]
fn ms_sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

/// スレッドに名前を付けて生成
///
/// TODO: 後で構造体にする
/// 使用例
/// ```
/// let mut threads: HashMap<&str, fn(Sender<String>, Sender<u16>)> = HashMap::new();
///
/// threads.insert("test", test);
///
/// let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) = mpsc::channel();
///   
/// let (sendr_msg, receiver_msg): (Sender<u16>, Receiver<u16>) = mpsc::channel();
///
/// thread_generate(threads, &sendr_err_handles, &sendr_msg);
///
/// fn test(panic_msg: Sender<String>, msg: Sender<u16>){}
///
/// ```
fn thread_generate(
    threads: HashMap<&str, fn(Sender<String>, Sender<u16>)>,
    err: &Sender<String>,
    msg: &Sender<u16>,
) {
    for (name, i) in threads {
        let sendr_join_handle_errmsg = mpsc::Sender::clone(err);
        let sendr_join_handle_msg = mpsc::Sender::clone(msg);

        let _thread = thread::Builder::new()
            .name(name.to_string())
            .spawn(move || {
                i(sendr_join_handle_errmsg, sendr_join_handle_msg);
            })
            .unwrap();
    }
}

/// 独自panicシステム
fn send_panic_msg(panic_msg: Sender<String>) {
    let default_hook: Box<dyn Fn(&panic::PanicInfo) + Sync + Send> = panic::take_hook();
    let m: Mutex<Sender<String>> = Mutex::new(panic_msg);

    panic::set_hook(Box::new(move |panic_info: &panic::PanicInfo| {
        let handle: thread::Thread = thread::current();

        let err_msg: std::sync::MutexGuard<Sender<String>> = m.lock().unwrap();

        err_msg.send(handle.name().unwrap().to_owned()).unwrap();

        default_hook(panic_info)
    }));
}

#[macro_export]
macro_rules! thread_generate {
    ( $( $x:expr ),* ) => {
        {
            $(

                thread::spawn(move ||  {
                    $x();
                });
            )*

        }
    };
}
