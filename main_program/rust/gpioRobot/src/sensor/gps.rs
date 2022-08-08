use nav_types::{ENU, WGS84};
use std::time::Duration;
use std::{thread, vec};

/// r#box
/// 0           1
/// +───────────+
/// │           │
/// │           │
/// │           │
/// │           │
/// +───────────+
/// 2           3

#[test]
fn test() {
    let mut latlot: Vec<(f64, f64)> = Vec::new();

    latlot.push((36.061899, 136.222481));
    latlot.push((36.061899, 136.222481));
    latlot.push((36.061899, 136.222481));
    latlot.push((36.061899, 136.222481));
    //latlot.push((36.061899, 137.222481));

    let mut tmp = GPSmodule {
        r: 0.001,
        latlot: latlot,
    };

    /*
    let mut count = 0;


    for i in 1..=(latlot.len() / 2) {


        let tmp = (latlot[count+1].0 - latlot[count].0 ,latlot[count+1].1 - latlot[count].1);



        println!("{:?}",tmp);

        count+=2;
    }


    */

    loop {
        tmp.nav((36.062024, 136.222473));
    }

    //GPSmodule::eazimuth((36.062024, 136.222473));
    //GPSmodule::eazimuth((36.062024, 50.222473));
}

struct GPSmodule {
    r: f64,
    latlot: Vec<(f64, f64)>,
}

impl GPSmodule {
    fn nav(&mut self, now_postion: (f64, f64)) {
        //let now_postion = ;

        println!("{:?}", self.latlot);

        if self.latlot.len() == 0 {
            println!("stop");

        } else {
            let pos_a = WGS84::from_degrees_and_meters(self.latlot[0].0, self.latlot[0].1, 0.0);
            let pos_b = WGS84::from_degrees_and_meters(now_postion.0, now_postion.1, 0.0);
            let vec = pos_b - pos_a;
            let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

            if self.r#box(self.latlot[0], now_postion, self.r) {
                println!("azimuth: {:?}", azimuth);
                self.latlot.remove(0);
            } else {
                println!("continue");
                //println!("azimuth: {:?}", azimuth);
            }
        }

        time_sleep(1);

        println!("nav");
    }

    fn nazimuth(b: (f64, f64)) {
        let pos_a = WGS84::from_degrees_and_meters(90.0, 0.0, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        println!("{}", azimuth);
    }

    fn eazimuth(b: (f64, f64)) {
        let pos_a = WGS84::from_degrees_and_meters(0.0, 90.0, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        println!("{}", azimuth);
    }

    /// .ボックス状の判定
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
}

#[inline]
fn time_sleep(sec: u64) {
    thread::sleep(Duration::from_secs(sec));
}
