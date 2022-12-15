use ::tui::backend::CrosstermBackend;
use ::tui::Terminal;
use flacon::{Event, FlaCon, Flags};
use getch;
use gps::{self, GPS};
use mytools::time_sleep;
use robot_gpio::Moter;
use robot_serialport::RasPico;
use rthred::{send, sendG, Rthd, RthdG};
use scheduler::Scheduler;
use robot::tui;
use crate::robot::{config, self};
use crate::robot::setting::Settings;
use crate::thread_variable;
use super::key::input_key;

use config::{
    SenderOrders
};


struct SRP<'a> {
    opcode:u32,
    bytes: &'a [u8],
}

#[derive(Debug, Clone)]
pub struct AutoEvents {
    pub is_debug: bool,
    pub is_break: bool,
    pub is_move: bool,
    pub is_trune: bool,
    pub opcode: u32,
    pub opcode_history: Vec<u32>,
    pub latlot: (f64, f64),
    pub first_time: bool,
    pub trun_azimuth: f64,
    pub is_continue: bool,
    pub maneuver: &'static str,
}



pub fn auto() {




}