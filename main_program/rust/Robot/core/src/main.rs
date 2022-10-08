extern crate flacon;
extern crate load_shdlib;
extern crate slam;

use std::cell::Cell;

use flacon::{Event, Events, FlaCon, Flags, Module};

mod robot;
use robot::{gpio, setting::Settings};

 fn main() {






    let setting_file = Settings::load_setting("./settings.yaml");



    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();


    
    let mut moter_controler = gpio::Moter::new(right_moter_pin, left_moter_pin);



    let module = Module {
        moter_controler,
        
    };



    let event = Events {
        is_move: Cell::new(false),
        is_trune: Cell::new(false),
        is_emergency_stop_lv1: Cell::new(false),
        is_emergency_stop_lv0: Cell::new(false),
    };



    let mut flag_controler = FlaCon::new(module, event);

    //tmp.event.is_move.set(true);

    flag_controler.add_fnc("move", |flacn| {
        if flacn.event.is_move.get() {
            println!("move");
        };
    });

    loop {
        //flag_controler.event.is_move.set(true);

        flag_controler.load_fnc("move");
    }
}


