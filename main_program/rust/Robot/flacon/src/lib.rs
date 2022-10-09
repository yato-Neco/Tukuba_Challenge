use std::cell::Cell;
use std::collections::HashMap;

use robot_gpio::Moter;

pub struct Module {
    pub moter_controler: Moter,
}

pub struct Events {
    pub is_debug: bool,
    pub is_move: Cell<bool>,
    pub is_trune: Cell<bool>,
    pub is_emergency_stop_lv1: Cell<bool>,
    pub is_emergency_stop_lv0: Cell<bool>,
    pub is_lidar_stop: Cell<bool>,
    pub order: Cell<u32>,
}

pub trait Flags {
    fn new(module: Module, event: Events) -> FlaCon;
    fn add_fnc(&mut self, name: &str, f: fn(_self: &FlaCon));
    fn none_fnc(_self: &FlaCon);
}

pub trait Event {
    fn load_fnc(&self, name: &str);
}
pub struct FlaCon {
    pub event: Events,
    pub model: Module,
    fnc_map: HashMap<String, fn(_self: &FlaCon)>,
}

impl Flags for FlaCon {
    fn new(module: Module, event: Events) -> Self {
        FlaCon {
            event: event,
            model: module,

            fnc_map: HashMap::new(),
        }
    }

    fn add_fnc(&mut self, name: &str, fnc_pointer: fn(_self: &Self)) {
        self.fnc_map.insert(name.to_owned(), fnc_pointer);
    }

    fn none_fnc(_self: &FlaCon) {
        panic!("Not Fnc")
    }
}

impl Event for FlaCon {
    fn load_fnc(&self, name: &str) {
        let tmp = match self.fnc_map.get(name) {
            Some(e) => *e,
            None => Self::none_fnc,
        };

        tmp(&self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
