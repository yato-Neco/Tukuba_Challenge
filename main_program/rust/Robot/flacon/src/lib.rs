use robot_gpio::Moter;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// フラグコントロール関係
///
///
/// ```
/// struct Module {}
/// let module = Module {};
///
/// strct Event {}
/// let event = Event {}
///
/// let mut controler = FlaCon::new(module, event);
///
/// controler.add_fnc("test", |flacn| {
///  flacn.module;
///  flacn.event;
///  println!("test");
/// });
/// ```
#[derive(Clone)]
pub struct FlaCon<T, R> {
    pub event: R,
    pub module: T,
    fnc_map: HashMap<&'static str, fn(_self: &mut FlaCon<T, R>)>,
    is_panic: bool,
}

pub trait Flags<T, R> {
    fn new(module: T, event: R) -> FlaCon<T, R>;
    fn set_panic(&mut self);
}

pub trait Event<T, R> {
    fn add_fnc(&mut self, name: &'static str, f: fn(_self: &mut FlaCon<T, R>));
    fn load_fnc(&mut self, name: &str);
    fn load_fnc_is(&mut self, name: &str, flag: bool);
    fn none_fnc(_self: &mut FlaCon<T, R>);
}

/// フラグコントロール関係の構造体
impl<T, R> Flags<T, R> for FlaCon<T, R> {
    fn new(module: T, event: R) -> Self {
        FlaCon {
            event: event,
            module: module,
            fnc_map: HashMap::new(),
            is_panic: false,
        }
    }

    fn set_panic(&mut self) {
        self.is_panic = true;
    }
}

/// フラグのイベント関係の構造体
impl<T, R> Event<T, R> for FlaCon<T, R> {
    /// HashMapにフラグの名前と関数ポインタを入れる。
    fn add_fnc(&mut self, name: &'static str, fnc_pointer: fn(_self: &mut Self)) {
        self.fnc_map.insert(name, fnc_pointer);
    }

 


    /// フラグのイベントを呼び出す関数
    fn load_fnc(&mut self, name: &str) {
        let tmp = match self.fnc_map.get(name) {
            Some(e) => *e,
            None => {
                if self.is_panic {
                    panic!("Not Fnc {}", name);
                } else {
                    Self::none_fnc
                }
            }
        };

        tmp(self);
    }

    fn load_fnc_is(&mut self, name: &str, flag: bool) {
        let tmp = match self.fnc_map.get(name) {
            Some(e) => *e,
            None => {
                if self.is_panic {
                    panic!("Not Fnc {}", name);
                } else {
                    Self::none_fnc
                }
            }
        };

        if flag {
            tmp(self);
        }
    }

    fn none_fnc(_self: &mut Self) {}
}
