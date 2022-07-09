
mod robot;
use robot::{moter::MotorGPIO};

use std::{thread, vec};
use std::time::Duration;


fn main() {

    let tmp = MotorGPIO::new([25,24], [0,0]);


    let v = vec![0,1];
    
    

    
    /*
    thread::spawn(||{
        

    });
    */

}

