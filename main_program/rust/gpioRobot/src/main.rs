use std::sync::mpsc;
use std::time::Duration;
use std::{thread, vec};

mod robot;
mod sensor;

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut handles = vec![];
    let mut handles_data = vec![];

    const SENSER_HANDLES_LEN:u32 = 4;


    for i in 0..SENSER_HANDLES_LEN {
        handles_data.push(mpsc::Sender::clone(&tx));
    }


    handles.push(s0(handles_data[0].to_owned()));
    handles.push(s1(handles_data[1].to_owned()));
    handles.push(s2(handles_data[2].to_owned()));
    handles.push(s3(handles_data[3].to_owned()));
    
    let handles_len = handles.len();
    
    
    let mut i = 0;



    for handle in handles {
        handle.join().expect(&format!("err handle index by {}",i));

        i+=1;
    }


    //time_sleep(3);


    for (i, received) in rx.iter().enumerate() {
        println!("{:?}", received);
        //println!("{}",i);
        if i == (handles_len - 1) {break};

    }
    


    Motor();
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


fn s0(tx:mpsc::Sender<Vec<u64>> ) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("s0");
        time_sleep(0);
        tx.send(vec![0]).unwrap();

    })
    
}

fn s1(tx:mpsc::Sender<Vec<u64>>) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        println!("s1");
        time_sleep(1);
        tx.send(vec![1]).unwrap();


    })

}

fn s2(tx:mpsc::Sender<Vec<u64>>) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        println!("s2");
        time_sleep(2);
        tx.send(vec![2]).unwrap();


    })

}


fn s3(tx:mpsc::Sender<Vec<u64>>) -> thread::JoinHandle<()> {
    thread::spawn(move|| {
        println!("s3");
        time_sleep(5);
        tx.send(vec![3]).unwrap();


    })
    
}

#[inline]
fn time_sleep(sec:u64) {
    thread::sleep(Duration::from_secs(sec));
}