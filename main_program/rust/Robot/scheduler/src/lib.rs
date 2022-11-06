use std::{
    ops::{Add, Sub},
    thread,
    time::{Duration, Instant},
};
//https://docs.rs/quanta/latest/quanta/ 使用検討

pub struct Scheduler {
    start_time: Instant,
    next_start_time: Option<Instant>,
}

/// Scheduler
impl Scheduler {
    // カウントを開始する。
    pub fn start() -> Scheduler {
        let now_time = Instant::now();

        Scheduler {
            start_time: now_time,
            next_start_time: None,
        }
    }




    


    /// もう一つのカウントを開始する
    pub fn add_time_counter(&mut self) {
        self.next_start_time = Some(Instant::now());
    }

    /// start_time の ms i128
    pub fn nowtime(&mut self) -> i128 {
        let end = self.start_time.elapsed();
        end.as_millis() as i128
    }

    /// 一つ目のカウントと二つ目のカウントの終わりの差を一つ目のカウントにセット
    pub fn end(&mut self) {
        // 二つ目のカウントが始まって終わった時の時間を、一つ目のカウントでは無かったことにする。
        self.start_time = self.start_time.add(self.next_start_time.unwrap().elapsed());

    }
}

#[test]
fn test() {
    let mut tmp = Scheduler::start();
    thread::sleep(Duration::from_millis(300));

    tmp.add_time_counter();

    thread::sleep(Duration::from_millis(300));

    tmp.end();

    println!("{} ", tmp.nowtime());

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
