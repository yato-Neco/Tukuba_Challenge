use std::cell::Cell;
use std::collections::HashMap;

use robot_gpio::Moter;
use gps::GPSmodule;






/// フラグコントロール関係
pub struct FlaCon<T,R> {
    pub event: R,
    pub module: T,
    fnc_map: HashMap<String, fn(_self: &mut FlaCon<T,R>)>,
}


pub trait Flags<T,R> {
    fn new(module: T, event: R) -> FlaCon<T,R>;
    fn add_fnc(&mut self, name: &str, f: fn(_self: &mut FlaCon<T,R>));
    fn none_fnc(_self: &FlaCon<T,R>);
}

pub trait Event {
    fn load_fnc(&mut self, name: &str);
}



/// フラグコントロール関係の構造体
impl<T,R> Flags<T,R> for FlaCon<T,R> {
    fn new(module: T, event: R) -> Self {
        FlaCon {
            event: event,
            module: module,
            fnc_map: HashMap::new(),
        }
    }

    /// HashMapにフラグの名前と関数ポインタを入れる。
    fn add_fnc(&mut self, name: &str, fnc_pointer: fn(_self: &mut Self)) {
        self.fnc_map.insert(name.to_owned(), fnc_pointer);
    }
    

    fn none_fnc(_self: &FlaCon<T,R>) {
    }
}


/// フラグのイベント関係の構造体
impl<T,R> Event for FlaCon<T,R> {

    /// フラグのイベントを呼び出す関数
    /// &mut を外したい
    fn load_fnc(&mut self, name: &str) {
        let tmp = match self.fnc_map.get(name) {
            Some(e) => *e,
            None => {
                panic!("Not Fnc {}",name);
            },
        };

        tmp(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
