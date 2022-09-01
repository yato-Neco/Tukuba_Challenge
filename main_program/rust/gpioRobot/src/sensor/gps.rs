use nav_types::{ENU, WGS84};
use std::time::Duration;
use std::{thread, vec};

//#[test]
fn gps_test() {
    let mut latlot: Vec<(f64, f64)> = Vec::new();

    latlot.push((36.061899, 136.222481));
    latlot.push((36.061899, 136.222481));
    latlot.push((36.061899, 136.222481));
    latlot.push((36.061899, 136.222481));
    //latlot.push((36.061899, 137.222481));

    let mut tmp = GPSmodule {
        r: 0.001,
        latlot: &mut latlot,
    };

    loop {
        tmp.nav((36.062024, 136.222473));
    }

    //GPSmodule::eazimuth((36.062024, 136.222473));
    //GPSmodule::eazimuth((36.062024, 50.222473));
}

#[test]
fn test2() {
    let pos_a = WGS84::from_degrees_and_meters(36.061899, 136.222481, 0.0);
    let pos_b = WGS84::from_degrees_and_meters(36.061899, 136.222482, 0.0);

    println!("Distance between a and b: {:.2}m", pos_a.distance(&pos_b));

    let vec = pos_b - pos_a;

    let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

    println!("{}", azimuth);
}

pub struct GPSmodule<'a> {
    pub r: f64,
    pub latlot: &'a mut Vec<(f64, f64)>,
}

///
///
impl GPSmodule<'_> {
    /// ナビの機能
    /// 小数点で計算すると誤差があるので整数にしてる
    ///
    pub fn nav(&mut self, now_postion: (f64, f64)) -> (bool, (f64, f64), (f64, f64)) {
        //println!("{:?}", self.latlot);

        let now_postion_int: (f64, f64) = (
            (now_postion.0 * (10.0_f64.powf(6.0))).round(),
            (now_postion.1 * (10.0_f64.powf(6.0))).round(),
        );

        //println!("{:?}", now_postion_int);

        let r_int: f64 = self.r * (10.0_f64.powf(6.0));

        let mut azimuth: f64 = 0.0_f64;
        let mut distance: f64 = 0.0_f64;

        match self.latlot.len() {
            0 => {
                return (true, (azimuth, distance), (0.0, 0.0));
            }
            1.. => {
                //println!("{:?}", self.latlot);
                let latlot_int: (f64, f64) = (
                    self.latlot[0].0 * (10.0_f64.powf(6.0)),
                    self.latlot[0].1 * (10.0_f64.powf(6.0)),
                );

                let flag: bool = self.r#box(latlot_int, now_postion_int, r_int);

                let diff: (f64, f64) = (
                    latlot_int.0 - now_postion_int.0,
                    latlot_int.1 - now_postion_int.1,
                );

                (azimuth, distance) = self.fm_azimuth_int(&now_postion_int);

                if flag {
                    self.latlot.remove(0);
                    return (false, (azimuth, distance), diff);
                } else {
                    return (false, (azimuth, distance), diff);
                }
            }
            _ => {
                return (false, (azimuth, distance), (0.0, 0.0));
            }
        }
    }

    /// TODO: 距離も追加
    /// int ver
    fn fm_azimuth_int(&self, now_postion_int: &(f64, f64)) -> (f64, f64) {
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

    /// TODO: 距離も追加
    ///
    fn fm_azimuth(&self, now_postion: &(f64, f64)) -> f64 {
        let pos_a = WGS84::from_degrees_and_meters(self.latlot[0].0, self.latlot[0].1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(now_postion.0, now_postion.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);

        println!("{}", distance);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        azimuth
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
    fn r#box(&self, latlon: (f64, f64), now_p: (f64, f64), r: f64) -> bool {
        if latlon == (0.0, 0.0) {
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

/*
#[inline]
fn time_sleep(sec: u64) {
    thread::sleep(Duration::from_secs(sec));
}
*/

/*
let mut count = 0;


for i in 1..=(latlot.len() / 2) {


    let tmp = (latlot[count+1].0 - latlot[count].0 ,latlot[count+1].1 - latlot[count].1);



    println!("{:?}",tmp);

    count+=2;
}
*/
