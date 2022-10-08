use std::cell::Cell;
use std::collections::HashMap;






pub struct Module<T> {
    pub moter_controler:T,

}



pub struct Events {
    pub is_move: Cell<bool>,
    pub is_trune: Cell<bool>,
    pub is_emergency_stop_lv1: Cell<bool>,
    pub is_emergency_stop_lv0: Cell<bool>,
}





pub trait Flags<T> {
    fn new(module:Module<T>,event:Events) -> FlaCon<T>;
    fn add_fnc(&mut self,name:&str ,f: fn(_self: &FlaCon<T>));
    fn none_fnc(_self: &FlaCon<T>);
}

pub trait Event {

    fn load_fnc(&self, name: &str);
}
pub struct FlaCon<T> {
    pub event: Events,
    pub model: Module<T>,
    fnc_map: HashMap<String, fn(_self: &FlaCon<T>)>,
}


impl <T> Flags<T> for FlaCon<T> {
    fn new(module:Module<T>,event:Events) -> Self {
        FlaCon {
            event:event,
            model: module,

            fnc_map: HashMap::new(),
        }
    }

    fn add_fnc(&mut self, name:&str ,fnc_pointer: fn(_self: &Self)) {
        self.fnc_map.insert(name.to_owned(), fnc_pointer);
    }

    fn none_fnc(_self: &FlaCon<T>) {
        panic!("Not Fnc")
    }
}

impl <T> Event for FlaCon<T> {
  

    

    fn load_fnc(&self, name: &str) {
        let tmp =  match self.fnc_map.get(name) {
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
