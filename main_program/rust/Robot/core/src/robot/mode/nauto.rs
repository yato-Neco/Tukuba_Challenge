use std::{collections::HashMap, sync::mpsc::{Sender, Receiver, self}, time::Duration, io::Stdout};

use flacon::{FlaCon, Flags, Event};
use gps::gps::Nav;
use mytools::time_sleep;
use robot_gpio::Moter;
use rthred::{sendG, send};
use tui::{Terminal, backend::CrosstermBackend};

use crate::{thread_variable, robot::{config::{SenderOrders, self}, setting::Settings}};

use super::test2::Scheduler;



#[derive(Debug, Clone)]
pub struct AutoEvents {
    pub is_core_stop: bool,
    pub is_debug: bool,
    pub is_break: bool,
    pub is_move: bool,
    pub is_trune: bool,
    pub is_first_time: bool,
    pub is_continue: bool,
    pub opcode: u32,
    pub azimuth: f64,
    pub maneuver: &'static str,
}

struct AutoModule {
    //pub terminal: Terminal<CrosstermBackend<Stdout>>,
    //pub moter_controler: Moter,
    pub nav: Nav,
    pub scheduler: Scheduler,
    pub send:for<'r> fn(u32, &'r Sender<u32>),
    msg:Sender<u32>
}

pub fn nauto() {

    let setting_file = Settings::load_setting("./settings.yaml");
    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();
    let mut moter_controler = Moter::new(right_moter_pin, left_moter_pin);


    let opcode = thread_variable!("operator");

    
    let mut thread: HashMap<&str, fn(Sender<String>, SenderOrders)> =
        std::collections::HashMap::new();
    
    let (sendr_err_handles, _receiver_err_handle): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread.insert("operator", operator);


    loop{
        
        match opcode.get("operator").unwrap().1.try_recv() {
            Ok(e) => {
                moter_controler.moter_control(e);
            }
            Err(_) => {}
        };

        time_sleep(0, 10);
    }

}



fn operator(panic_msg: Sender<String>, msg: SenderOrders) {
    let setting_file = Settings::load_setting("./settings.yaml");
    let gps_setting = setting_file.load_gps_serial();

    let module = AutoModule {
        nav: Nav::init(),
        scheduler: Scheduler::start(),
        send:send,
        msg:msg,
    };

    let event = AutoEvents {
        is_core_stop: false,
        is_break: false,
        is_continue: false,
        is_debug: false,
        is_move: false,
        is_trune: false,
        is_first_time: true,
        opcode: 0xfffffff,
        azimuth: 0.0,
        maneuver: "Start",
    };

    let mut flacn = FlaCon::new(module, event);


    let mut gps_port = match serialport::new(gps_setting.0, gps_setting.1)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
    {
        Ok(p) => p,
        Err(_) => panic!(),
    };

    let mut gps_serial_buf: Vec<u8> = vec![0; gps_setting.2];

    //waypoint設定 -->
    let mut waypoints = Vec::new();
    waypoints.push((36.064225, 136.221375));
    flacn.module.nav.add_waypoints(waypoints);
    //<--

    flacn.add_fnc("first_time", |flacn| {
        flacn.event.maneuver = "first_time";
        (flacn.module.send)(config::FRONT, &flacn.module.msg);

        if flacn.module.nav.lat_lon_history.len() > 1 {

            
            flacn.event.azimuth =
            flacn.module.nav.frist_calculate_azimuth();

            (flacn.module.send)(config::STOP, &flacn.module.msg);


            flacn.event.is_first_time = false;

        }
        
        
    });
    
    
    loop {

        match gps_port.read(gps_serial_buf.as_mut_slice()) {
            Ok(t) => {
                let gps_data = String::from_utf8_lossy(&gps_serial_buf[..t]).to_string();
                flacn.module.nav.gps_senser.parser(gps_data);
                if flacn.module.nav.gps_senser.is_fix {
                    flacn.module.nav.set_lat_lot(flacn.module.nav.gps_senser.lat_lon.unwrap());
                }
            }
            Err(_) => {}
        }

        
        flacn.load_fnc_is("first_time",flacn.module.nav.gps_senser.is_fix && flacn.event.is_first_time);
        flacn.module.nav.robot_move(0.0, 0.0);
        

        if !flacn.event.is_first_time {
            continue;
        }




        if flacn.module.nav.is_in_waypoint() {
            break;
        }

        //send(config::FRONT, &msg);
        

    }


}


fn serial() {
    


    
}