use std::{
    collections::HashMap,
    io::{Stdout, Write},
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use flacon::{Event, FlaCon, Flags};
use gps::gps::Nav;
use mytools::{ms_sleep, time_sleep};
use robot_gpio::Moter;
use rthred::{send, sendG, Rthd};
use tui::{backend::CrosstermBackend, Terminal};
use wt901::WT901;
use crate::robot::setting::Settings;
use super::key::input_key;


pub fn shitunaiyou() {
    let setting_file = Settings::load_setting("./settings.yaml");
    let (right_moter_pin, left_moter_pin) = setting_file.load_moter_pins();



    
}