use nav_types::{ENU, WGS84};
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::{thread, vec};

#[test]
fn gps_test() {
    let mut latlot: Vec<(f64, f64)> = Vec::new();

    latlot.push((36.061899, 136.222481));

    //latlot.push((36.061899, 137.222481));

    let mut tmp = GPSmodule {
        r: 0.001,
        latlot: latlot,
    };

    loop {
        tmp.nav((36.062024, 136.222473));
    }

    //GPSmodule::eazimuth((36.062024, 136.222473));
    //GPSmodule::eazimuth((36.062024, 50.222473));
}

#[test]
fn test2() {
    let pos_a = WGS84::from_degrees_and_meters(36.000_006, 136.000_006, 0.0);
    let pos_b = WGS84::from_degrees_and_meters(36.000_000, 136.000_000, 0.0);

    println!("Distance between a and b: {:.4}m", pos_a.distance(&pos_b));

    let vec = pos_b - pos_a;

    let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

    println!("{}", azimuth);
}

#[derive(Debug)]
pub struct GPSmodule {
    pub r: f64,
    pub latlot: Vec<(f64, f64)>,
}

pub struct GPS {
    pub port: &'static str,
    buf: Vec<u8>,
    pub nowpotion: Option<(f64, f64)>,
    pub noepotion_history: Vec<(f64, f64)>,
    pub r: f64,
    pub latlot: Vec<(f64, f64)>,
}

#[test]
fn test() {
    let mut tmp = GPS::new("", 500);
    tmp.latlot.push((0.001, 0.001));
    tmp.nowpotion = Some((0.001, 0.001));
    tmp.nav();
}

impl GPS {
    fn new(port_name: &'static str, buf_size: usize) -> Self {
        Self {
            port: port_name,
            buf: Vec::with_capacity(buf_size),
            nowpotion: None,
            noepotion_history: Vec::new(),
            r: 0.001,
            latlot: Vec::new(),
        }
    }

    fn parser(&self) {
        /*
        while self.nowpotion == None {
            match self.nowpotion {
                Some(e) => (),
                None => (),
            }
        }
        */
    }

    fn nav(&mut self) {
        /*
        let now_postion_int: (f64, f64) = (
            (self.nowpotion.unwrap().0 * (10.0_f64.powf(6.0))).round(),
            (self.nowpotion.unwrap().1 * (10.0_f64.powf(6.0))).round(),
        );

        let r_int: f64 = self.r * (10.0_f64.powf(6.0));

        println!("{:?} {:?}",now_postion_int,r_int);
        */

        let flag: bool = self.r#box(&(self.latlot[0].0,self.latlot[0].1), &self.nowpotion.unwrap(), self.r);

        println!("{}",flag);

        let result = match self.latlot.len() {
            0 => {

            }
            1.. => {

            }
            _ => {

            }
        };

        let (azimuth, distance) = self.fm_azimuth(&self.nowpotion.unwrap());

        println!("{:?} {:?}", azimuth, distance);
    }

    fn fm_azimuth(&self, now_postion: &(f64, f64)) -> (f64, f64) {
        let pos_a = WGS84::from_degrees_and_meters(self.latlot[0].0, self.latlot[0].1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(now_postion.0, now_postion.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);

        println!("{}", distance);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        (azimuth, distance)
    }

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
///
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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}