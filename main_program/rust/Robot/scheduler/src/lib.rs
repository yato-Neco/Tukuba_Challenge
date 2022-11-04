use std::{
    ops::{Add, Sub},
    thread,
    time::{Duration, Instant},
};

pub struct Scheduler {
    pub time: i128,
    start_time: Instant,
    next_start_time: Option<Instant>,
}

/// Scheduler
impl Scheduler {
    // カウントを開始する。
    pub fn start() -> Scheduler {
        let now_time = Instant::now();

        Scheduler {
            time: 0_i128,
            start_time: now_time,
            next_start_time: None,
        }
    }

    /// i128 に変換して返す。
    pub fn end(&mut self) -> i128 {
        let end = self.start_time.elapsed();
        self.time = end.as_millis() as i128;

        self.time
    }

    /// もう一つのカウントの ms を i128で返す。
    pub fn end2(&mut self) -> i128 {
        let end = self.next_start_time.unwrap().elapsed();
        self.time = end.as_millis() as i128;

        self.time
    }

    pub fn int_time() {}

    /// もう一つのカウントを開始する
    pub fn add(&mut self) {
        self.next_start_time = Some(Instant::now());
    }

    /// 非推奨
    fn _checked_sub(&self) -> i128 {
        let a = self.start_time.add(self.next_start_time.unwrap().elapsed());

        //let a = self.start_time.checked_add(self.next_start_time.unwrap().elapsed()).unwrap();
        a.elapsed().as_millis() as i128
    }

    /// 一つ目のカウントと二つ目のカウントの終わりの差を一つ目のカウントにセット
    pub fn checked_sub(&mut self) {
        // 二つ目のカウントが始まって終わった時の時間を、一つ目のカウントでは無かったことにする。
        let a = self.start_time.add(self.next_start_time.unwrap().elapsed());
        self.start_time = a;

        //let a = self.start_time.checked_add(self.next_start_time.unwrap().elapsed()).unwrap();
    }
}



#[test]
fn test() {
    let mut tmp = Scheduler::start();
    thread::sleep(Duration::from_millis(300));

    tmp.add();

    thread::sleep(Duration::from_millis(300));

    tmp.checked_sub();

    println!("{} ", tmp.end());

    /*
    loop {
        thread::sleep(Duration::from_millis(100));
        let result1 = tmp.end();
        let result2 = tmp.end2();
        let result = tmp.checked_sub();
        println!("{} {} {}",result1,result2,result);
    }
    */
}
