extern crate flacon;
extern crate load_shdlib;
extern crate slam;

use std::cell::Cell;

use flacon::{Event, Events, FlaCon, Flags, Module};

mod robot;
use robot::{setting::Settings, mode::Mode,config};
use robot_gpio::Moter;


#[tokio::main]
async  fn main() {


    let setting_file = Settings::load_setting("./settings.yaml");



    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();


    
    let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);


    let mut mode = Mode {};

    let module = Module {
        moter_controler,
        
        
    };



    let event = Events {
        is_debug: false,
        is_move: Cell::new(false),
        is_trune: Cell::new(false),
        is_emergency_stop_lv1: Cell::new(false),
        is_emergency_stop_lv0: Cell::new(false),
        is_lidar_stop: Cell::new(false),
        order: Cell::new(0xfffffff),

    };



    let mut flag_controler = FlaCon::new(module, event);

    flag_controler.event.is_move.set(true);

    flag_controler.add_fnc("lidar-stop", |flacn| {
        if flacn.event.is_lidar_stop.get() {



        };
    });

    flag_controler.add_fnc("move", |flacn| {
        if flacn.event.is_move.get() {
            println!("move");
        };
    });

    
    
    loop {

        //flag_controler.event.is_move.set(true);
        let order = match Mode::key()  {
            config::BREAK => break,
            o => o,
        };

        
        println!("{}",order);

        //flag_controler.event.order.set(order);

        

        //flag_controler.load_fnc("move");
        //flag_controler.load_fnc("lidar-stop");


    }
}


