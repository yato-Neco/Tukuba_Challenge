use std::{
    thread,
    time::{Duration, Instant},
};

pub struct Benchmark {
    pub start_time: Instant,
}

/// プログラムの実行スピードを測る
///
/// ```
/// let mut time = Benchmark::start();
///
/// time.end();
///
/// ```
impl Benchmark {
    pub fn start() -> Benchmark {
        let now_time = Instant::now();
        Benchmark {
            start_time: now_time,
        }
    }

    pub fn endprln(&self) {
        let end = self.start_time.elapsed();
        println!(
            "Process {}.{:03} msec",
            end.as_micros() / 1000,
            end.as_micros() % 1000,
        );
    }

    pub fn endf64(&self) -> f64 {
        let end = self.start_time.elapsed();
        end.as_secs_f64()
    }

    pub fn endu32(&self) -> i128 {
        let end = self.start_time.elapsed();
        end.as_millis() as i128
    }
}

#[inline]
pub fn time_sleep(sec: u64, ms: u64) {
    thread::sleep(Duration::from_secs(sec));
    thread::sleep(Duration::from_millis(ms));
}

#[inline]
pub fn mic_sleep(micro: u64) {
    thread::sleep(Duration::from_micros(micro));
}

/// warning: の部分が黄色になる。
#[inline]
pub fn warning_msg(txt: &str) {
    //println!("{}{}", "Warning: ".red(), txt);
    println!("\x1b[{}mWarning:\x1b[m {}", 33, txt)
}

#[inline]
/// 非推奨
pub fn ms_sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

#[test]
fn test() {
    warning_msg("test");

    let tmp2 = (11.0 - 4.0) * 0.1;
    println!("{:?}", tmp2);

    let tmp = roundf(tmp2, 10);

    println!("{}", tmp);
}

///
/// 四捨五入
#[inline]
pub fn roundf(x: f64, square: i32) -> f64 {
    (x * (square as f64)).round() / (square as f64)
}

pub trait Xtools {
    fn roundf(&self, square: i32) -> f64;
}

pub trait XtoolsBool<T> {
    fn conditional(&self, r: T, t: T) -> T;
}

impl<T> XtoolsBool<T> for bool {
    fn conditional(&self, arg1: T, arg2: T) -> T {
        if *self {
            arg1
        } else {
            arg2
        }
    }
}

impl Xtools for f64 {
    fn roundf(&self, square: i32) -> f64 {
        (self * (square as f64)).round() / (square as f64)
    }
}


#[cfg(test)]
mod tests {
    use crate::XtoolsBool;


    #[test]
    fn it_works() {
        let a = true;

        let s = a.conditional("a", "b");

        println!("{}",s);

    }
}