use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;



struct MotorGPIO {
    r_pin0: OutputPin,
    r_pin1: OutputPin,

    l_pin0: OutputPin,
    l_pin1: OutputPin,
}

impl MotorGPIO {

    
    pub fn new(r_pin: [u8; 2], l_pin: [u8; 2]) -> MotorGPIOs {

        let mut r_pin0: OutputPin = Gpio::new().unwrap().get(self.r_pin[0]).unwrap().into_output();
        let mut r_pin1: OutputPin = Gpio::new().unwrap().get(self.r_pin[1]).unwrap().into_output();

        let mut l_pin0: OutputPin = Gpio::new().unwrap().get(self.l_pin[0]).unwrap().into_output();
        let mut l_pin1: OutputPin = Gpio::new().unwrap().get(self.l_pin[1]).unwrap().into_output();


        return MotorGPIO{
            r_pin0,
            r_pin1,

            l_pin0,
            l_pin1,
        }

    }

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
