use std::sync::mpsc;
use std::time::Duration;
use std::{thread, vec};

mod robot;
mod senser;

fn main() {
    let (tx, rx) = mpsc::channel();

    let mut handles = vec![];

    let lider = thread::spawn(move || {
        tx.send("t").unwrap();
        println!("lider");
    });

    let motor = thread::spawn(|| {
        println!("motor");

    });

    handles.push(lider);
    handles.push(motor);

    let mut i = 0;




    for handle in handles {
        handle.join().expect(&format!("err handle index by {}",i));

        i+=1;
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
    senser::tflite::python().unwrap();
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
