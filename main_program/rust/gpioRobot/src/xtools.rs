use std::{thread, time::{Duration, Instant}};


pub struct Benchmark {
    start_time: Instant,
}

impl Benchmark {
    pub fn start() -> Benchmark {
        let now_time = Instant::now();
        Benchmark {
            start_time: now_time,
        }
    }

    pub fn end(&self) {
        let end = self.start_time.elapsed();
        println!(
            "Process {}.{:03} msec",
            end.as_micros() / 1000,
            end.as_micros() % 1000,
        );
    }
}


#[inline]
pub fn time_sleep(sec: u64) {
    thread::sleep(Duration::from_secs(sec));
}

#[inline]
pub fn ms_sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
