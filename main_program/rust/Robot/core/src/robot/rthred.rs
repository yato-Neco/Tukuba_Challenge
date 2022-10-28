use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex};
use std::{panic, thread};


use crate::xtools::{warning_msg};
use crate::robot::config::SenderOrders;

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

pub struct Rthd<T:'static + std::marker::Send> {
    l:T
}

pub trait Rthds  {
    fn thread_generate(
        threads: HashMap<&str, fn(Sender<String>, SenderOrders)>,
        err_msg: &Sender<String>,
        msg: &SenderOrders,
    );
    fn _thread_generate();
    fn send_panic_msg(panic_msg: Sender<String>) ;
    fn send(order: u32, msg: &SenderOrders);
}   

impl<T: 'static + std::marker::Send> Rthd<T> {
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
        threads: HashMap<&str, fn(Sender<String>, SenderOrders,)>,
        err_msg: &Sender<String>,
        msg: &HashMap<&str, (Sender<u32>, Receiver<u32>)>
        ,
    ) {
        for (name, fnc) in threads {
            let sendr_join_handle_errmsg = mpsc::Sender::clone(err_msg);
            let sendr_join_handle_msg = mpsc::Sender::clone(&msg.get(name).expect(name).0);
            let _thread = thread::Builder::new()
                .name(name.to_string())
                .spawn(move || {
                    fnc(sendr_join_handle_errmsg, sendr_join_handle_msg, );
                })
                .unwrap();
        }
    }


    /// ジェネリック型　thread_generater
    pub fn _thread_generate(name:&str,err_msg: &Sender<String>,sender:Sender<T>,func:fn(Sender<String>,Sender<T>)) {
        let sendr_join_handle_errmsg = mpsc::Sender::clone(err_msg);

        let _thread = thread::Builder::new()
        .name(name.to_string())
        .spawn(move || {
            func(sendr_join_handle_errmsg,sender);
        })
        .unwrap();
        
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


/// 必要ないしｓ
pub struct RthdG<T: 'static + std::marker::Send,R:>{
    t:T,
    r:R
}

impl<T: 'static + std::marker::Send,R: 'static + std::marker::Send> RthdG<T,R> {
    /// senderをreturnするthread_generater
    pub fn _thread_generate_return_sender(name:&str,err_msg: &Sender<String>,sender:Sender<T>,arg:R,func:fn(Sender<String>,Sender<T>,R))  {
        let sendr_join_handle_errmsg = mpsc::Sender::clone(err_msg);
        //let return_sender;
        let _thread = thread::Builder::new()
        .name(name.to_string())
        .spawn(move || {
            func(sendr_join_handle_errmsg,sender,arg);
        })
        .unwrap();


        //return return_sender;
    }
}

#[inline]
    pub fn send(order: u32, msg: &SenderOrders) {
        match msg.send(order) {
            Ok(_) => (),
            Err(_) => warning_msg("Can not send msg"),
        };
    }

