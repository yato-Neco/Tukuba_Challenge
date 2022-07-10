
use std::os::windows::fs::OpenOptionsExt;

use std::{thread, vec};
use std::time::Duration;

mod robot;

fn main() {

        
    thread::spawn(||{


    });
}


#[test]
fn py_test(){

    robot::tflite::python().unwrap();

}

#[cfg(target_os = "linux")]
#[test]
fn Motor() {
    use robot::{moter::MotorGPIO};
    let tmp = MotorGPIO::new([25,24], [0,0]);

}