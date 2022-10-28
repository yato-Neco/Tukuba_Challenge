use yaml_rust::Yaml;

use crate::robot::settings::Settings;
use crate::xtools::{roundf, time_sleep, Benchmark};
use crate::robot::moter::{MoterGPIO};

pub struct DisplayMode {
    pub operation: Vec<(u32, u32)>,
}

#[test]
fn test() {
    let settings_path: &str = "./settings.yaml";

    let settings_yaml = Settings::load_setting(settings_path);

    let mut tmp = DisplayMode::new();
    let mut moter = MoterGPIO::new([25,24],[22,23]);
    tmp.load_csv(&settings_yaml);

    loop {
        let order = match tmp.start() {
            Some(e) => e,
            None => break,
        };
        let s = Benchmark::start();
        //println!("{:X} {}", order.0, order.1);

        MoterGPIO::moter_control(order.0,&mut moter);
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
    pub fn load_csv(&mut self,settings_yaml: &Yaml) {
        extern crate csv;
        use std::fs::File;



        let file = File::open(settings_yaml["Robot"]["Display_mode"]["order"][0].as_str().unwrap()).unwrap();
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


