use std::{
    thread,
    time::{Duration, Instant},
};

pub struct Benchmark {
    start_time: Instant,
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

    pub fn end(&self) -> f64 {
        let end = self.start_time.elapsed();
        end.as_secs_f64()
    }
}

#[inline]
pub fn time_sleep(sec: u64, ms: u64) {
    thread::sleep(Duration::from_secs(sec));
    thread::sleep(Duration::from_millis(ms));
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
    //println!("{:?}",tmp2);

    let tmp = roundf(0.0015, 1000);


    println!("{}",tmp);

}


///
/// 四捨五入
#[inline]
pub fn roundf(x:f64,square:i32) -> f64 {
     (x * (square as f64)).round() / (square as f64)
}