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

    /*
    構造体には初期時に呼び出される関数がないため、
    自ら作る必要がある

    new()　は引数のGPIO Pin の初期化を担当。
    そして return としてMotorGPIOをすることによって main関数の変数 tmp で MotorGPIOのメゾットが呼び出せる。
    */
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

    /*
    前進の関数
    引数にselfがる。
    ここら辺はPythonとなんら変わりないが、引数の &mut selfは
    & が値を借りることを意味する(借りることを明示的に宣言している)
    mut は値を変えることを宣言している
    // https://doc.rust-jp.rs/book-ja/ch04-02-references-and-borrowing.html

    Rust の基本として必ず値を返す時、型を明示しなければならない。
    (-> Result<(), Box<dyn Error>>)

    結果をResultで包むことによって try catch を実装しなくてもよくなる

    
    */
    pub fn forward(&mut self)  -> Result<(), Box<dyn Error>>  {
        //rigth motor

        
        
        self.r_pin0.set_low();
        self.r_pin1.set_high();
        
        self.l_pin0.set_low();
        self.l_pin1.set_high();



        Ok(())
    }


    pub fn backward(&mut self) -> Result<(), Box<dyn Error>> {


        self.r_pin0.set_high();
        self.r_pin1.set_low();
        
        self.l_pin0.set_high();
        self.l_pin1.set_low();


        Ok(())

    }




    pub fn pivotTurnRight(&mut self){

        self.r_pin0.set_high();
        self.r_pin1.set_high();
        
        self.l_pin0.set_high();
        self.l_pin1.set_low();

    }


    pub fn pivotTurnLeft(&mut self){

        self.r_pin0.set_high();
        self.r_pin1.set_low();
        
        self.l_pin0.set_high();
        self.l_pin1.set_high();
        
    }


    pub fn turnRight(&mut self){

        self.r_pin0.set_low();
        self.r_pin1.set_high();
        
        self.l_pin0.set_high();
        self.l_pin1.set_low();

    }


    pub fn turnLeft(&mut self){

        self.r_pin0.set_high();
        self.r_pin1.set_low();
        
        self.l_pin0.set_low();
        self.l_pin1.set_high();
        
    }


    

    /*
    pub fn right_motor(&self) -> Result<(), Box<dyn Error>> {
        let mut pin0: OutputPin = Gpio::new()?.get(self.r_pin[0])?.into_output();
        let mut pin1: OutputPin = Gpio::new()?.get(self.r_pin[1])?.into_output();

        
        pin0.set_low();
        pin1.set_high();
        

        Ok(())
    }

    pub fn left_motor(&self) -> Result<(), Box<dyn Error>> {
        let mut pin0: OutputPin = Gpio::new()?.get(self.l_pin[0])?.into_output();
        let mut pin1: OutputPin = Gpio::new()?.get(self.l_pin[1])?.into_output();

        pin0.set_low();
        pin1.set_high();

        Ok(())
    }
    
    */
    
}
