use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex};
use std::{panic, thread};

use yaml_rust::Yaml;

use crate::SenderOrders;
use crate::xtools::{warning_msg};

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

pub struct Rthd {}

pub trait Rthds  {
    fn thread_generate(
        threads: HashMap<&str, fn(Sender<String>, SenderOrders)>,
        err_msg: &Sender<String>,
        msg: &SenderOrders,
    ); 
    fn send_panic_msg(panic_msg: Sender<String>) ;
    fn send(order: u32, msg: &SenderOrders);
}   

impl Rthd {
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
    pub fn thread_generate(
        threads: HashMap<&str, fn(Sender<String>, SenderOrders, Yaml)>,
        err_msg: &Sender<String>,
        msg: &SenderOrders,
        settings_yaml: Yaml
    ) {
        for (name, fnc) in threads {
            let sendr_join_handle_errmsg = mpsc::Sender::clone(err_msg);
            let sendr_join_handle_msg = mpsc::Sender::clone(msg);
            let s = settings_yaml.clone();
            let _thread = thread::Builder::new()
                .name(name.to_string())
                .spawn(move || {
                    fnc(sendr_join_handle_errmsg, sendr_join_handle_msg, s);
                })
                .unwrap();
        }
    }

    /// 独自panicシステム
    ///
    ///
    ///
    /// ```
    /// send_panic_msg("painc!");
    /// ```
    ///
    pub fn send_panic_msg(panic_msg: Sender<String>) {
        let default_hook: Box<dyn Fn(&panic::PanicInfo) + Sync + Send> = panic::take_hook();
        let m: Mutex<Sender<String>> = Mutex::new(panic_msg);

        panic::set_hook(Box::new(move |panic_info: &panic::PanicInfo| {
            let handle: thread::Thread = thread::current();

            let err_msg: std::sync::MutexGuard<Sender<String>> = m.lock().unwrap();

            err_msg.send(handle.name().unwrap().to_owned()).unwrap();

            default_hook(panic_info)
        }));
    }

    
}

#[inline]
    pub fn send(order: u32, msg: &SenderOrders) {
        match msg.send(order) {
            Ok(_) => (),
            Err(_) => warning_msg("Can not send msg"),
        };
    }

/// Deprecated API
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
