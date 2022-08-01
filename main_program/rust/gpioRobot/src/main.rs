use std::collections::HashMap;
use std::panic;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, vec};

mod robot;
mod sensor;



/// ┌────────┐    ┌────────┐
/// │thread:1│    │thread:2│
/// └───┬────┘    └────┬───┘
///     │              │
///     └──────┬───────┘
///            │
///            │
/// ┌──────────▼───────────┐     ┌─────────────────────────┐
/// │                      │     │     send_panic_msg      │
/// │  thread_generate()   │     │                         │
/// │                      │     │ ┌─────────┐ ┌─────────┐ │
/// └──────────┬───────────┘     │ │thread1()│ │thread2()│ ├──┐
///            │                 │ └─────────┘ └─────────┘ │  │
///            │                 │                         │  │
///            │                 │      thread_spwan       │  │panic!
///            │                 │                         │  │
///            │                 └────────────▲────────────┘  │
///            │                              │               │
///            └──────────────────────────────┘               │
///                                                           │
///                                                           │
///                                                        ┌──▼─┐
///                                                        │main│
///                                                        └────┘


fn main() {
    //println!("{:?}",contacts);

    let mut contacts: HashMap<&str, fn(Sender<String>, Sender<String>)> = HashMap::new();

    //let mut tname = Vec::new();

    //let (tx, rx) = mpsc::channel();

    contacts.insert("name-s3", s3);
    contacts.insert("name-s4", s4);
    //contacts.insert("name-s5", s5);

    /*
    for (name, i) in contacts {
        let tx1 = mpsc::Sender::clone(&tx);

        let _thread =  thread::Builder::new().name(name.to_string()).spawn(move || {
            i(tx1);
        }).unwrap();

        tname.push(_thread);

    }

    */

    let (sendr_err__handles, receiver_err_handle): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    let (sendr_msg, receiver_msg): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    let handle_len = contacts.len();

    thread_generate(contacts.clone(), &sendr_err__handles, &sendr_msg);

    //thread_generate!(s4);

    //let mut count = 1;
    //let mut tmp = Vec::new();

    loop {
        //println!("{}",count);
        let rethread = contacts.clone();

        //println!("{}",receiver_join_handle.recv().unwrap());

        for j in receiver_msg.try_recv() {
            println!("{}", j);
        }


        println!("main");

        time_sleep(1);
    }

    //println!("------------------------------------");

    //time_sleep(1);

    Motor();
}

#[test]
fn test() {
    panic::set_hook(Box::new(|panic_info| {
        println!("test");
        println!("{}", panic_info);
    }));

    panic!("Normal panic");
}

//#[test] はpy_test()だけを動かすことができる
#[test]
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
pub fn Motor() {}

fn s3(panic_msg: Sender<String>, msg: Sender<String>) {
    let mut c = 0;

    send_panic_msg(panic_msg);

    loop {
        if c > 0 {
            panic!("panic")
        };
        println!("s3 is moved");
        time_sleep(10);
        //c += 1;

        msg.send("re".to_owned()).unwrap();
    }
}

fn s4(panic_msg: Sender<String>, msg: Sender<String>) {
    send_panic_msg(panic_msg);
    let mut c = 0;

    loop {
        if c > 0 {
            panic!("panic")
        };

        println!("s4 is moved");
        //panic!();
        time_sleep(1);
        //c += 1;
    }
}

fn s5(panic_msg: Sender<String>) {
    send_panic_msg(panic_msg);
    let mut c = 0;

    loop {
        if c > 0 {
            panic!("panic")
        };

        println!("s5 is moved");
        time_sleep(1);
        //panic!();
        //c += 1;
    }
}
#[inline]
fn time_sleep(sec: u64) {
    thread::sleep(Duration::from_secs(sec));
}

// スレッド生成
fn thread_generate(
    threads: HashMap<&str, fn(Sender<String>, Sender<String>)>,
    err: &Sender<String>,
    msg: &Sender<String>,
)
/* ->   (Vec<&str>, std::sync::mpsc::Receiver<String>)*/
{
    //let mut join_handle: Vec<thread::JoinHandle<()>> = Vec::new();

    /*

    let (sendr_join_handles, receiver_join_handle): (Sender<String>, Receiver<String>) =
        mpsc::channel();

    let join_handle_name: Vec<&str> = threads
        .to_owned()
        .into_iter()
        .map(|(name, _value)| name)
        .collect();

    */

    //println!("{:?}",join_handle_name);

    for (name, i) in threads {
        let sendr_join_handle_errmsg = mpsc::Sender::clone(err);
        let sendr_join_handle_msg = mpsc::Sender::clone(msg);

        let _thread = thread::Builder::new()
            .name(name.to_string())
            .spawn(move || {
                i(sendr_join_handle_errmsg, sendr_join_handle_msg);
            })
            .unwrap();

        //join_handle.push(_thread);
    }

    //(join_handle_name, receiver_join_handle)
}

//独自panicシステム
fn send_panic_msg(panic_msg: Sender<String>) {
    let default_hook: Box<dyn Fn(&panic::PanicInfo) + Sync + Send> = panic::take_hook();
    let m: Mutex<Sender<String>> = Mutex::new(panic_msg);

    panic::set_hook(Box::new(move |panic_info: &panic::PanicInfo| {
        let handle: thread::Thread = thread::current();

        let t: std::sync::MutexGuard<Sender<String>> = m.lock().unwrap();

        t.send(handle.name().unwrap().to_owned()).unwrap();

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
