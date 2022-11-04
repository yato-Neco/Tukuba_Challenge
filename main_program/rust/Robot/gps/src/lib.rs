use core::num;
use nav_types::{ENU, WGS84};
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::{thread, vec};


#[test]
fn gps() {
    let mut gps = GPS::new("COM4", 115200, 1000);

    let result = gps.nav();

    println!("{}", result);

    GPS::_serial("COM5", 115200, 1000);
}

#[test]
fn test() {
    let mut tmp = GPS::new("COM4", 115200, 500);
    tmp.latlot.push((0.001, 0.001));
    tmp.nowpotion = Some((0.001, 0.001));

    loop {
        println!("{:?}", tmp.latlot);
        let result: bool = tmp.nav();
        println!("{}", result);
        if !result {
            break;
        }
    }
}


#[derive(Debug)]
pub struct GPSmodule {
    pub r: f64,
    pub latlot: Vec<(f64, f64)>,
}
#[derive(Debug, Clone)]
pub struct GPS {
    pub port: String,
    pub rate: u32,
    pub buf_size: usize,
    pub nowpotion: Option<(f64, f64)>,
    pub original_nowpotion: String,
    pub noepotion_history: Vec<(f64, f64)>,
    pub azimuth: f64,
    pub now_azimuth: Option<f64>,
    pub distance: f64,
    pub r: f64,
    pub is_fix: Option<bool>,
    pub num_sat: Option<usize>,
    pub latlot: Vec<(f64, f64)>,
    pub in_waypoint: bool,
    pub next_latlot: Option<(f64, f64)>,
}



impl GPS {
    pub fn new(port: &str, rate: u32, buf_size: usize) -> Self {
        Self {
            port: port.to_string(),
            rate: rate,
            buf_size: buf_size,
            nowpotion: None,
            original_nowpotion: String::new(),
            noepotion_history: Vec::new(),
            azimuth: 0.0,
            now_azimuth: None,
            distance: 0.0,
            r: 0.001,
            is_fix: None,
            num_sat: None,
            latlot: Vec::new(),
            next_latlot: None,
            in_waypoint:false,
        }
    }

    /// Serialからデータを受け取って、main スレッドに送る。
    pub fn serial(port: &str, rate: u32, buf_size: usize, msg: Sender<String>) {
        let mut port = match serialport::new(port, rate)
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Eight)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(p) => (p),
            Err(_) => (panic!()),
        };

        let mut serial_buf: Vec<u8> = vec![0; buf_size];
        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    //serial_buf[..t].to_vec();
                    let gps_data = String::from_utf8_lossy(&serial_buf[..t]).to_string();

                    msg.send(gps_data).unwrap();
                }
                Err(_) => {}
            }
        }
    }

    /// 非推奨
    pub fn _serial(port: &str, rate: u32, buf_size: usize) {
        let mut port = match serialport::new(port, rate)
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Eight)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(p) => (p),
            Err(_) => (panic!()),
        };

        let mut serial_buf: Vec<u8> = vec![0; buf_size];
        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Ok(t) => {
                    //serial_buf[..t].to_vec();
                    let mut tmp = GPS::new("COM4", 115200, 500);

                    let gps_data = String::from_utf8_lossy(&serial_buf[..t]).to_string();
                    println!("{}",gps_data);
                    tmp.parser(gps_data);
                    println!("{:?} {:?} {:?} {:?}",tmp.nowpotion, tmp.is_fix, tmp.num_sat ,tmp.original_nowpotion);
                    //msg.send(gps_data).unwrap();
                }
                Err(_) => {}
            }
        }
    }

    ///
    /// シリアル通信から来るデータ扱いやすく、パースする。
    pub fn parser(&mut self, gps_data: String) {
        let gps_format = gps_data.replace(' ', "");

        let vec: Vec<&str> = gps_format.split(&[':', '=', ','][..]).collect();
        //print!("{:?}",v);
        //println!("{:?}",vec.iter().find(|&&num_sat| num_sat == " numSat"),);
        let gps_format = match vec.iter().find(|&&num_sat| num_sat == "numSat") {
            Some(_) => {
                //println!("{} {:?}",i,gps_format[i].split(',').collect::<Vec<&str>>());

                gps_format.split(',').collect::<Vec<&str>>()
            }
            None => [].to_vec(),
        };

        /*
        println!("{:?}",gps_format);
        println!("{:?}", gps_format.get(1));
        println!("{:?}", gps_format.get(2));
        println!("{:?}", gps_format.get(3));
        println!("{:?}", gps_format.get(4));
        */

        match gps_format.get(1) {
            Some(e) => {
                let num_sat = e.split_at(7);
                self.num_sat = num_sat.1.parse::<usize>().ok();
            }
            None => {}
        }

        match gps_format.get(2) {
            Some(e) => {
                if *e == "No-Fix" {
                    self.is_fix = Some(false);
                } else if *e == "Fix" {
                    self.is_fix = Some(true);
                }
            }
            None => {}
        };

        match gps_format.get(3) {
            Some(lat) => {
                let lot = gps_format.get(4).unwrap();

                self.nowpotion = Some((lat.parse::<f64>().unwrap(), lot.parse::<f64>().unwrap()))
            }
            None => {}
        }

        //println!("{:?}", self);
    }

    /// 古い方のparser
    ///
    pub fn _parser(&mut self, gps_data: String) {

        let mut gps_format = Vec::new();

        gps_format.push("SpGnss : begin in".to_owned());
        gps_format.push("SpGnss : begin out".to_owned());
        gps_format.push("SpGnss : begin out".to_owned());
        gps_format.push("mode = HOT_START".to_owned());
        gps_format.push("SpGnss : start out".to_owned());
        gps_format.push("Gnss setup OK".to_owned());
        gps_format.push("1980/01/06 00:00:03.000626, numSat: 0, No-Fix, 0.0, 0.0,".to_owned());

        let gps_format = gps_format
            .iter_mut()
            .map(|n| n.replace(' ', ""))
            .collect::<Vec<String>>();

        for (i, v) in gps_format.iter().enumerate() {
            let vec: Vec<&str> = v.split(&[':', '=', ','][..]).collect();
            //print!("{:?}",v);
            //println!("{:?}",vec.iter().find(|&&num_sat| num_sat == " numSat"),);
            let gps_format = match vec.iter().find(|&&num_sat| num_sat == "numSat") {
                Some(_) => {
                    //println!("{} {:?}",i,gps_format[i].split(',').collect::<Vec<&str>>());

                    gps_format[i].split(',').collect::<Vec<&str>>()
                }
                None => [].to_vec(),
            };

            //println!("{:?}",gps_format);
            println!("{:?}", gps_format.get(1));
            println!("{:?}", gps_format.get(2));
            println!("{:?}", gps_format.get(3));
            println!("{:?}", gps_format.get(4));

            // retrun gps_format
            // ↓　別

            match gps_format.get(1) {
                Some(e) => {
                    let num_sat = e.split_at(7);
                    self.num_sat = num_sat.1.parse::<usize>().ok();
                }
                None => {}
            }

            match gps_format.get(2) {
                Some(e) => {
                    if *e == "No-Fix" {
                        self.is_fix = Some(false);
                    } else if *e == "Fix" {
                        self.is_fix = Some(true);
                    }
                }
                None => {}
            };

            match gps_format.get(3) {
                Some(lat) => {
                    let lot = gps_format.get(4).unwrap();

                    self.nowpotion =
                        Some((lat.parse::<f64>().unwrap(), lot.parse::<f64>().unwrap()))
                }
                None => {}
            }

            println!("{:?}", self);
        }
    }

    /// ロボットが実際に動くことをシミュレートする
    pub fn running_simulater(&mut self,arg:bool) {
        if arg {
            //println!("{:?}",self.latlot[0].0 > self.nowpotion.unwrap().0);
            if self.latlot[0].0 > self.nowpotion.unwrap().0 {
                //self.nowpotion.unwrap().0 -= 0.001;
                self.nowpotion = Some((
                    roundf(self.nowpotion.unwrap().0 + 0.001, 1000),
                    self.nowpotion.unwrap().1,
                ));
            } else if self.latlot[0].0 < self.nowpotion.unwrap().0 {
                self.nowpotion = Some((
                    roundf(self.nowpotion.unwrap().0 - 0.001, 1000),
                    self.nowpotion.unwrap().1,
                ));
            }
            if self.latlot[0].1 > self.nowpotion.unwrap().1 {
                self.nowpotion = Some((
                    self.nowpotion.unwrap().0,
                    roundf(self.nowpotion.unwrap().1 + 0.001, 1000),
                ));
            } else if self.latlot[0].1 < self.nowpotion.unwrap().1 {
                self.nowpotion = Some((
                    self.nowpotion.unwrap().0,
                    roundf(self.nowpotion.unwrap().1 - 0.001, 1000),
                ));
            }
        }
    }

    /// nav システム
    /// 戻り値は終了時のbool
    pub fn nav(&mut self) -> bool {
        /*
        let now_postion_int: (f64, f64) = (
            (self.nowpotion.unwrap().0 * (10.0_f64.powf(6.0))).round(),
            (self.nowpotion.unwrap().1 * (10.0_f64.powf(6.0))).round(),
        );

        let r_int: f64 = self.r * (10.0_f64.powf(6.0));

        println!("{:?} {:?}",now_postion_int,r_int);
        */

        //let len_flag: bool = self.latlot.len() == 0;

        let result = match self.latlot.len() {
            0 => false,
            1.. => {
                let box_flag: bool = self.r#box(
                    &(self.latlot[0].0, self.latlot[0].1),
                    &self.nowpotion.unwrap(),
                    self.r,
                );

                (self.azimuth, self.distance) = self.fm_azimuth(&self.nowpotion.unwrap());

                //println!("{:?} {:?}", azimuth, distance);

                //println!("{}",box_flag);

                self.running_simulater(true);

                self.in_waypoint = box_flag;

                if box_flag {
                    self.latlot.remove(0);
                    //self.next_latlot = Some((self.latlot[0].0, self.latlot[0].1));
                }


                true
            }
            _ => false,
        };

        //println!("{}",result);
        result
    }

    /// 二地点間の角度(度数法)
    fn fm_azimuth(&self, now_postion: &(f64, f64)) -> (f64, f64) {
        let pos_a = WGS84::from_degrees_and_meters(self.latlot[0].0, self.latlot[0].1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(now_postion.0, now_postion.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);

        //println!("{}", distance);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        (azimuth, distance)
    }


    /// 設定したlatlotに半径(box状だけど)r に入った true 以外 false
    fn r#box(&self, latlon: &(f64, f64), now_p: &(f64, f64), r: f64) -> bool {
        if *latlon == (0.0, 0.0) {
            panic!("緯度経度設定しろ！")
        };

        let lat0_bottom = latlon.0 - r;
        let lat0_top = latlon.0 + r;

        let lon1_bottom = latlon.1 - r;
        let lon1_top = latlon.1 + r;

        if (lat0_bottom <= now_p.0 && now_p.0 <= lat0_top)
            && (lon1_bottom <= now_p.1 && now_p.1 <= lon1_top)
        {
            return true;
        }

        return false;
    }
}

///
/// 旧GPS
impl GPSmodule {
    /// ナビの機能
    /// 小数点で計算すると誤差があるので整数にしてる
    /// TODO: 整数にする所にバグとコンピューターリソースに問題あり？
    /// nav()のリターン (終了フラグ), 角度, 距離, 現在位置と目的位置の差分)
    pub fn nav(
        &mut self,
        now_postion: (f64, f64),
    ) -> (bool, (f64, f64), (f64, f64), bool, Vec<(f64, f64)>) {
        let now_postion_int: (f64, f64) = (
            (now_postion.0 * (10.0_f64.powf(6.0))).round(),
            (now_postion.1 * (10.0_f64.powf(6.0))).round(),
        );

        //println!("{:?}", now_postion_int);

        let r_int: f64 = self.r * (10.0_f64.powf(6.0));

        let mut azimuth: f64 = 0.0_f64;
        let mut distance: f64 = 0.0_f64;

        let result: (bool, (f64, f64), (f64, f64), bool, Vec<(f64, f64)>) = match self.latlot.len()
        {
            0 => (
                true,
                (azimuth, distance),
                (0.0, 0.0),
                false,
                self.latlot.clone(),
            ),
            1.. => {
                //println!("{:?}", self.latlot);
                let latlot_int: (f64, f64) = (
                    self.latlot[0].0 * (10.0_f64.powf(6.0)),
                    self.latlot[0].1 * (10.0_f64.powf(6.0)),
                );

                let flag: bool = self.r#box(&latlot_int, &now_postion_int, r_int);

                let diff: (f64, f64) = (
                    latlot_int.0 - now_postion_int.0,
                    latlot_int.1 - now_postion_int.1,
                );

                (azimuth, distance) = self.fm_azimuth_int(&now_postion_int);
                //println!("{:?}",azimuth);

                if flag {
                    self.latlot.remove(0);
                    //println!("{}", "-".repeat(20));
                    //println!("ウェイポイント {:?}", self.latlot);
                    (false, (azimuth, distance), diff, flag, self.latlot.clone())
                } else {
                    (false, (azimuth, distance), diff, false, self.latlot.clone())
                }
            }
            _ => (
                false,
                (azimuth, distance),
                (0.0, 0.0),
                false,
                self.latlot.clone(),
            ),
        };

        result
    }

    /// TODO: 距離も追加
    /// int ver
    fn fm_azimuth_int(&self, now_postion_int: &(f64, f64)) -> (f64, f64) {
        //println!("{:?}",now_postion_int);
        //println!("{:?}",(self.latlot[0].0, self.latlot[0].1));
        let pos_a = WGS84::from_degrees_and_meters(self.latlot[0].0, self.latlot[0].1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(
            now_postion_int.0 / (10.0_f64.powf(6.0)).round(),
            now_postion_int.1 / (10.0_f64.powf(6.0)).round(),
            0.0,
        );

        let distance: f64 = pos_a.distance(&pos_b);
        //println!("Distance between a and b: {:.2}m", pos_a.distance(&pos_b));

        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        (azimuth, distance)
    }

    /// TODO: 距離も追加</br>
    /// 非推奨
    fn fm_azimuth(&self, now_postion: &(f64, f64)) -> (f64, f64) {
        let pos_a = WGS84::from_degrees_and_meters(self.latlot[0].0, self.latlot[0].1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(now_postion.0, now_postion.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);

        println!("{}", distance);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        (azimuth, distance)
    }

    /// .ボックス状の判定
    ///
    /// r#box
    /// 0           1
    /// +───────────+
    /// │           │
    /// │           │
    /// │           │
    /// │           │
    /// +───────────+
    /// 2           3
    fn r#box(&self, latlon: &(f64, f64), now_p: &(f64, f64), r: f64) -> bool {
        if *latlon == (0.0, 0.0) {
            panic!("緯度経度設定しろ！")
        };

        let lat0_bottom = latlon.0 - r;
        let lat0_top = latlon.0 + r;

        let lon1_bottom = latlon.1 - r;
        let lon1_top = latlon.1 + r;

        if (lat0_bottom <= now_p.0 && now_p.0 <= lat0_top)
            && (lon1_bottom <= now_p.1 && now_p.1 <= lon1_top)
        {
            return true;
        }

        return false;
    }

    ///
    /// load_waypointの引数はファイルパス</br>
    /// CSVファイルをロード</br>
    /// ※Excelでcsvファイルを操作するな!
    /// ```
    /// let path:&str = "./waypoint";
    ///
    /// let mut gps = GPSmodule{};
    ///
    /// gps.load_waypoint(path);
    ///
    /// ```
    /*


        pub fn load_waypoint(&mut self,path:&str) {
            extern crate csv;
            use std::fs::File;

            let file = File::open(path).unwrap();
            let mut rdr = csv::Reader::from_reader(file);
            println!("waypoint");
            for (i, result) in rdr.records().enumerate() {
                let record = result.expect("a CSV record");

                let slat = match record.get(0) {
                    Some(e) => e,
                    None => panic!("{}行目 latの設定", i),
                };
                let slot = match record.get(1) {
                    Some(e) => e,
                    None => panic!("{}行目 lotの設定", i),
                };

                println!("{:?}", (slat, slot));

                let lat: f64 = match slat.trim().replace("_", "").parse() {
                    Ok(e) => e,
                    Err(_) => panic!("{}行目 latがf64形式じゃないよ", i),
                };

                let lot: f64 = match slot.trim().replace("_", "").parse() {
                    Ok(e) => e,
                    Err(_) => panic!("{}行目 lotがf64形式じゃないよ", i),
                };

                self.latlot.push((lat, lot));
            }
        }
    */
    pub fn rotate(azimuth: f64, now_azimuth: &mut f64, msg: &Sender<u32>) {
        let c2: f64 = 10.0_f64.powf(2.0);

        let isr: bool = azimuth >= 0.0;
        let isl: bool = azimuth <= 0.0;
        let _azimuth = (azimuth * c2).round() / c2;

        if isr == isl {
            //send(STOP,&msg);
        } else if isr != isl {
            //send(0x1F2AFFFF, &msg);
            loop {
                let _now_azimuth = (*now_azimuth * c2).round() / c2;
                //println!("{}",_now_azimuth);

                if isr == isl || _azimuth == _now_azimuth {
                    //println!("回転終了");
                    break;
                }
                // 回転シュミ系

                GPSmodule::rotate_simulater(isr, now_azimuth);
            }

            //send(order::STOP, &msg);
        }
    }

    pub fn distance_con(index: usize) -> u32 {
        //let t1: f64 = 10.0_f64.powf(-6.0);
        //println!("{}", index);
        match index {
            000000 => 0x1F00FFFF,
            1..=300000 => 0x1F88FFFF,
            400000..=600000 => 0x1FAAFFFF,
            700000..=900000 => 0x1FCCFFFF,
            1000000..=1200000 => 0x1FDDFFFF,
            1300000.. => 0x1FEEFFFF,

            _ => 0x1FFFFFFF,
        }
    }

    #[inline]
    pub fn running_simulater(nlatlot: &mut (f64, f64), diff: &(f64, f64)) {
        let t1: f64 = 10.0_f64.powf(-6.0);
        if (diff.0 * t1) > 0.0 {
            nlatlot.0 += 0.000_001;
        } else if (diff.0 * t1) < 0.0 {
            nlatlot.0 -= 0.000_001;
        }

        if (diff.1 * t1) > 0.0 {
            //println!("z");
            nlatlot.1 += 0.000_001;
        } else if (diff.1 * t1) < 0.0 {
            nlatlot.1 -= 0.000_001;
        }
    }

    pub fn rotate_simulater(isr: bool, now_azimuth: &mut f64) {
        if isr {
            *now_azimuth += 0.1;
            //time_sleep(0, 5)
        } else {
            *now_azimuth -= 0.1;
            //time_sleep(0, 5)
        }
    }

    /// 非安定API
    fn nazimuth(b: (f64, f64)) {
        let pos_a = WGS84::from_degrees_and_meters(90.0, 0.0, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        println!("{}", azimuth);
    }

    /// 非安定API
    fn eazimuth(b: (f64, f64)) {
        let pos_a = WGS84::from_degrees_and_meters(0.0, 90.0, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        println!("{}", azimuth);
    }
}


/// 小数点切り捨て
#[inline]
pub fn roundf(x: f64, square: i32) -> f64 {
    (x * (square as f64)).round() / (square as f64)
}
