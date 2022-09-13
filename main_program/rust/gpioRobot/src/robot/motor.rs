/*

use 系はpython のimport


*/

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;



/*

構造体の宣言
Pythonでいう
def __init___():
    pass
の引数

https://doc.rust-jp.rs/book-ja/ch05-00-structs.html
*/

pub struct MotorGPIO {
    pub r_pin0: OutputPin,
    pub r_pin1: OutputPin,

    pub l_pin0: OutputPin,
    pub l_pin1: OutputPin,
}

impl MotorGPIO {

    /// GPIO のピンをセットする関数
    pub fn new(r_pin: [u8; 2], l_pin: [u8; 2]) -> MotorGPIO {

        let mut r_pin0: OutputPin = Gpio::new().unwrap().get(r_pin[0]).unwrap().into_output();
        let mut r_pin1: OutputPin = Gpio::new().unwrap().get(r_pin[1]).unwrap().into_output();

        let mut l_pin0: OutputPin = Gpio::new().unwrap().get(l_pin[0]).unwrap().into_output();
        let mut l_pin1: OutputPin = Gpio::new().unwrap().get(l_pin[1]).unwrap().into_output();


        return MotorGPIO{
            r_pin0,
            r_pin1,

            l_pin0,
            l_pin1,
        }

    }


  



    /// right モーター 前後
     /// duty 0.0 ~ 1.0
     pub fn rfpwm(&mut self, duty: f64) {
        self.r_pin1.set_pwm_frequency(50.0, duty).unwrap();
        self.r_pin0.set_pwm_frequency(0.0, 0.0).unwrap();
    }

    /// right モーター　後進
    /// duty 0.0 ~ 1.0
    pub fn rbpwm(&mut self, duty: f64) {
        self.r_pin1.set_pwm_frequency(0.0, 0.0).unwrap();
        self.r_pin0.set_pwm_frequency(50.0, duty).unwrap();
    }

    /// left モーター 前後
    /// duty 0.0 ~ 1.0
    pub fn lfpwm(&mut self, duty: f64) {
        self.l_pin1.set_pwm_frequency(50.0, duty).unwrap();
        self.l_pin0.set_pwm_frequency(0.0, 0.0).unwrap();
    }

    /// left モーター　後進
    /// duty 0.0 ~ 1.0
    pub fn lbpwm(&mut self, duty: f64) {
        self.l_pin1.set_pwm_frequency(0.0, 0.0).unwrap();
        self.l_pin0.set_pwm_frequency(50.0, duty).unwrap();
    }

    /// PWMのリセット
    pub fn pwm_all_clean(&mut self) {
        self.r_pin0.clear_pwm().unwrap();
        self.r_pin1.clear_pwm().unwrap();
        self.l_pin0.clear_pwm().unwrap();
        self.l_pin1.clear_pwm().unwrap();
    }
    
}
