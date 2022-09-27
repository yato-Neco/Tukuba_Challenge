use crate::xtools::{roundf, time_sleep, Benchmark};
use crate::robot::moter::{MoterGPIO};

pub struct DisplayMode {
    pub operation: Vec<(u32, u32)>,
}

#[test]
fn test() {
    let mut tmp = DisplayMode::new();
    let mut moter = MoterGPIO::new([25,24],[22,23]);
    tmp.load_csv();

    loop {
        let order = match tmp.start() {
            Some(e) => e,
            None => break,
        };
        let s = Benchmark::start();
        //println!("{:X} {}", order.0, order.1);
        println!("{:?}",moter_control(order.0,&mut moter));
        time_sleep(0, order.1 as u64);
        println!("{}", roundf(s.end(), 100));
    }
}

impl DisplayMode {
    pub fn new() -> DisplayMode {
        let operation: Vec<(u32, u32)> = Vec::new();
        Self {
            operation: operation,
        }
    }
    /// ロボットを動かすためのcsvを読み込む
    ///
    pub fn load_csv(&mut self) {
        extern crate csv;
        use std::fs::File;

        let file = File::open("order.csv").unwrap();
        let mut rdr = csv::Reader::from_reader(file);
        for (i, result) in rdr.records().enumerate() {
            let record = result.expect("a CSV record");

            let sorder = match record.get(0) {
                Some(e) => e,
                None => panic!("{}行目 の設定", i),
            };
            let stime = match record.get(1) {
                Some(e) => e,
                None => panic!("{}行目 の設定", i),
            };

            if sorder.len() <= 2 {
                panic!("len > 2");
            };

            let (front, back) = sorder.split_at(2);
            if front != "0x" {
                panic!("not use 0x");
            };

            let order: u32 = match u32::from_str_radix(&back, 16) {
                Ok(e) => e,
                Err(_) => panic!("{}行目 がu32形式じゃないよ", i),
            };

            let time: u32 = match stime.trim().replace("_", "").parse() {
                Ok(e) => e,
                Err(_) => panic!("{}行目 がu32形式じゃないよ", i),
            };

            self.operation.push((order, time));
        }
    }

    pub fn start(&mut self) -> Option<(u32, u32)> {
        let mut order = None;

        if self.operation.len() > 0 {
            let orders: (u32, u32) = self.operation[0];

            order = Some(orders);

            self.operation.remove(0);
        }

        order
    }
}


fn moter_control(order: u32, moter:&mut MoterGPIO)  {
    let rM: i8 = ((order & 0x00F00000) >> 20) as i8;
    let lM: i8 = ((order & 0x000F0000) >> 16) as i8;
     match (rM, lM) {
        (1..=7, 1..=7) => {
            println!("後進 {} {}", (rM - 8).abs(), (lM - 8).abs());
            moter.rbpwm(roundf((rM - 8).abs() as f64 * 0.1,10));
            moter.lbpwm(roundf((lM - 8).abs() as f64 * 0.1, 10));
        }
        (8..=14, 8..=14) => {
            println!("前進 {} {}", rM - 4, lM - 4);
            moter.rfpwm(roundf((rM - 4) as f64 * 0.1, 10));
            moter.lfpwm(roundf((lM - 4) as f64 * 0.1, 10));

        }
        (0, 0) => {
            println!("ストップ");
            moter.pwm_all_clean();
        }
        (1..=7, 8..=14) => {
            println!("回転 {} {}", (rM - 8).abs(), lM - 4);
            moter.rbpwm(roundf((rM - 8).abs() as f64 * 0.1, 10));
            moter.lfpwm(roundf((lM - 4) as f64 * 0.1, 10));

        }
        (8..=14, 1..=7) => {
            println!("回転 {} {}", rM - 4, (lM - 8).abs());
            moter.rfpwm(roundf((rM - 4) as f64 * 0.1, 10));
            moter.lbpwm(roundf((lM - 8).abs() as f64 * 0.1,10));
      
        }
        _ => {
            //println!("その他 {} {}", rM, lM);
            moter.pwm_all_clean();
        }
    };

}

