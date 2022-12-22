use mytools::{time_sleep, Xtools};
use nav_types::{ENU, WGS84};
use rthred::sendG;
use std::sync::mpsc::Sender;
use std::time::Duration;
use std::vec;

#[test]
fn test() {
    let tmp = GPS::new(false);
    //35.631316,139.330911
    let lat = &(35.631316, 139.330911);
    let lot = &(35.631317, 139.330912);
    let distance = tmp.distance(lat, lot);
    let azimuth = tmp.azimuth(lat, lot).round().abs();
    //println!("{}", distance);
    //println!("{}", azimuth);
    let latlot = vec![
        (35.631316, 139.330911),
        (35.631316, 139.330911),
        (35.631316, 139.330911),
    ];
    let siten = (35.631316, 139.330911);
    GPS::generate_rome(siten,latlot);

    //println!("{:?}",tmp.azimuth_none_gps(lat,lot));
}

#[derive(Debug, Clone)]
pub struct GPS {
    pub nowpotion: Option<(f64, f64)>,
    pub original_nowpotion: String,
    pub nowpotion_history: Vec<(f64, f64)>,
    pub azimuth: f64,
    pub now_azimuth: Option<f64>,
    pub distance: f64,
    r: f64,
    pub is_fix: Option<bool>,
    pub num_sat: Option<usize>,
    pub latlot: Vec<(f64, f64)>,
    pub in_waypoint: bool,
    pub next_latlot: Option<(f64, f64)>,
    pub is_simulater: bool,
    pub rome: Vec<(f64, f64)>,
    pub is_nowpotion_history_sub: bool,
    pub nowtime: String,
    pub gps_format: Vec<String>,
    pub guess_position: Option<(f64, f64)>,
    pub gps_msec: u32,
    pub m_ms: f64,
    pub wt901: WT901,
}

#[derive(Debug, Clone)]
pub struct WT901 {
    pub ang: Option<(f32, f32, f32)>,
    pub mag: Option<(u32, u32, u32)>,
}

impl GPS {
    pub fn new(simulater: bool) -> Self {
        let is_fix = if simulater { Some(true) } else { None };

        let nowpotion = if simulater {
            Some((36.000000, 136.000000))
        } else {
            None
        };

        Self {
            nowpotion: nowpotion,
            original_nowpotion: String::new(),
            nowpotion_history: Vec::new(),
            azimuth: 0.0,
            now_azimuth: None,
            distance: 0.0,
            r: 0.0002,
            is_fix: is_fix,
            num_sat: None,
            latlot: Vec::new(),
            next_latlot: None,
            in_waypoint: false,
            is_simulater: simulater,
            rome: Vec::with_capacity(100),
            is_nowpotion_history_sub: simulater,
            nowtime: String::new(),
            gps_format: Vec::new(),
            guess_position: None,
            gps_msec: 1000,
            m_ms: 0.0,
            wt901: WT901 {
                ang: None,
                mag: None,
            },
        }
    }

    /*
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
                    sendG(gps_data, &msg);
                    //msg.send(gps_data).unwrap();
                }
                Err(_) => {}
            }
        }
    }
    */

    ///
    /// シリアル通信から来るデータ扱いやすく、パースする。
    pub fn parser(&mut self, gps_data: String) {
        let gps_format = gps_data.replace(' ', "");

        let vec: Vec<&str> = gps_format.split(&[':', '=', ','][..]).collect();
        //print!("{:?}",v);
        //println!("{:?}",vec.iter().find(|&&num_sat| num_sat == " numSat"),);
        let mut gps_format = match vec.iter().find(|&&num_sat| num_sat == "numSat") {
            Some(_) => {
                //println!("{} {:?}",i,gps_format[i].split(',').collect::<Vec<&str>>());

                gps_format.split(',').collect::<Vec<&str>>()
            }
            None => [].to_vec(),
        };
        self.nowtime = gps_format.get(0).unwrap_or(&"").to_string();
        match gps_format.get(1) {
            Some(e) => {
                let num_sat = e.split_at(7);
                self.num_sat = num_sat.1.parse::<usize>().ok();
            }
            None => {}
        }

        //println!("{:?}",self.num_sat);
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

                self.nowpotion = Some((lat.parse::<f64>().unwrap(), lot.parse::<f64>().unwrap()));
                //unwarp err 改善

                //unwrap_or(self.nowpotion_history.last().unwrap().1)
                /*
                self.nowpotion = Some((
                    lat.parse::<f64>().ok().unwrap(),
                    lot.parse::<f64>().ok().unwrap(),
                )); //unwarp err 改善
                */
            }
            None => {}
        }

        self.gps_format = gps_format.iter_mut().map(|x| x.to_string()).collect();

        //println!("{:?}", self);
    }

    #[inline]
    /// ロボットが実際に動くことをシミュレートする
    pub fn running_simulater(&mut self, arg: bool) {
        if arg {
            if self.latlot[0].0 > self.nowpotion.unwrap().0 {
                self.nowpotion = Some((
                    (self.nowpotion.unwrap().0 + 0.001).roundf(1000),
                    //roundf(self.nowpotion.unwrap().0 + 0.001, 1000),
                    self.nowpotion.unwrap().1,
                ));
            } else if self.latlot[0].0 < self.nowpotion.unwrap().0 {
                self.nowpotion = Some((
                    (self.nowpotion.unwrap().0 - 0.001).roundf(1000),
                    self.nowpotion.unwrap().1,
                ));
            }
            if self.latlot[0].1 > self.nowpotion.unwrap().1 {
                self.nowpotion = Some((
                    self.nowpotion.unwrap().0,
                    (self.nowpotion.unwrap().1 + 0.001).roundf(1000),
                ));
            } else if self.latlot[0].1 < self.nowpotion.unwrap().1 {
                self.nowpotion = Some((
                    self.nowpotion.unwrap().0,
                    (self.nowpotion.unwrap().1 - 0.001).roundf(1000),
                ));
            }

            time_sleep(1, 0);
        }
    }

    /// nav システム
    /// 戻り値は終了時のbool
    pub fn nav(&mut self) -> bool {
        let result = match self.latlot.len() {
            0 => false,
            1.. => {
                let box_flag: bool = self.r#box(
                    &(self.latlot[0].0, self.latlot[0].1),
                    &self.nowpotion.unwrap(),
                    self.r,
                );

                (self.azimuth, self.distance) = self.fm_azimuth(&self.nowpotion.unwrap());

                self.running_simulater(self.is_simulater);

                self.in_waypoint = box_flag;

                if box_flag {
                    self.latlot.remove(0);
                }

                true
            }
            _ => false,
        };

        result
    }

    /// 二地点間の角度(度数法) と 距離
    fn fm_azimuth(&self, now_postion: &(f64, f64)) -> (f64, f64) {
        let pos_a = WGS84::from_degrees_and_meters(self.latlot[0].0, self.latlot[0].1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(now_postion.0, now_postion.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);

        //println!("{}", distance);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        (azimuth, distance)
    }

    ///  frist 実行時 二地点間の角度(度数法)
    pub fn frist_calculate_azimuth(&self) -> f64 {
        if self.is_simulater {
            return 0.0;
        }
        let nowpotion = self.nowpotion.unwrap();
        let pos_a = WGS84::from_degrees_and_meters(
            self.nowpotion_history[0].0,
            self.nowpotion_history[0].1,
            0.0,
        );
        let pos_b = WGS84::from_degrees_and_meters(nowpotion.0, nowpotion.1, 0.0);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);
        azimuth
    }

    pub fn nowpotion_history_sub(&self) -> bool {
        if self.is_simulater {
            return true;
        }

        if self.nowpotion_history.len() > 0 {
            let nowpotion = self.nowpotion.unwrap();
            let lat_sub = (nowpotion.0 - self.nowpotion_history[0].0).abs();
            let lot_sub = (nowpotion.1 - self.nowpotion_history[0].1).abs();

            if lat_sub > 0.00005 || lot_sub > 0.00005 {
                return true;
            }
        }

        return false;
    }

    /// 任意 二地点間の角度(度数法)
    pub fn calculate_azimuth(&self, latlot: (f64, f64)) -> f64 {
        let nowpotion = self.nowpotion.unwrap();
        let pos_a = WGS84::from_degrees_and_meters(nowpotion.0, nowpotion.1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(latlot.0, latlot.1, 0.0);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);
        azimuth
    }

    #[inline]
    pub fn distance(&self, a: &(f64, f64), b: &(f64, f64)) -> f64 {
        //let nowpotion = self.nowpotion.unwrap();
        let pos_a = WGS84::from_degrees_and_meters(a.0, a.1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);

        distance
    }

    #[inline]
    ///a,bの角度
    pub fn azimuth(&self, a: &(f64, f64), b: &(f64, f64)) -> f64 {
        let pos_a = WGS84::from_degrees_and_meters(a.0, a.1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        azimuth
    }

    pub fn azimuth_none_gps(&self, a: &(f64, f64), b: &(f64, f64)) {

        //let tmp = ((a.0 * 1000_000.0 - b.0 * 1000_000.0).abs(), (a.1 * 1000_000.0 - b.1 * 1000_000.0).abs());
        //let tmp2 = (0.0,0.0);

        //let ab_vec = ((a.0 * b.0) + (a.1 * b.1)) / ((a.0.powf(2.0) + a.1.powf(2.0)).sqrt() * (b.0.powf(2.0) + b.1.powf(2.0)).sqrt());

        //println!("{:?}",ab_vec);
    }

    pub fn generate_rome(siten:(f64,f64),latlot:Vec<(f64, f64)>) {

        let mut distance_vec = Vec::new();

        for p in latlot {
            let pos_a = WGS84::from_degrees_and_meters(p.0, p.1, 0.0);
            let pos_b = WGS84::from_degrees_and_meters(p.0, p.1, 0.0);
            let distance: f64 = pos_a.distance(&pos_b);
            distance_vec.push(distance);
        }

        distance_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        println!("{:?}",distance_vec);

        let mut start_mesh_map = (0, 0);

        let cm = (0.03 * 100.0) as u64 * 2;
        // 1cm

        //println!("{}", cm / 2);

        let mut mesh_map = Vec::new();

        //let mut mesh_map_y = Vec::new();

        for y in 0..=cm {
            let mut mesh_map_x = Vec::new();

            for x in 0..=cm {
                //println!("{} {}",y,x);
                mesh_map_x.push(0);
            }
            mesh_map.push(mesh_map_x);
        }

        //mesh_map.push(mesh_map_x);

        for i in mesh_map {
            println!("{:?}", i);
        }

        // 1cm
    }

    pub fn _generate_rome(&mut self) {
        let len = self.latlot.len() - 1;

        //println!("{}",self.latlot[0].0 - self.latlot[1].0);

        for i in 0..len {
            let sub0: f64 = (self.latlot[i].0 - self.latlot[i + 1].0).roundf(1000_000);
            let sub1: f64 = (self.latlot[i].1 - self.latlot[i + 1].1).roundf(1000_000);

            //println!("{:?} : {:?}",self.latlot[i],self.latlot[i + 1]);
            //println!("{}",sub0);

            if sub0.abs() >= 0.000009 || sub1.abs() >= 0.000009 {
                let x = (self.latlot[i].0 + (sub0 / 2.0)).roundf(1000_000);

                if self.latlot[i] == self.latlot[i + 1] {
                    let y = self.latlot[i].1;
                    self.rome.push((x, y))
                } else {
                    let mut y = (((self.latlot[i + 1].1 - self.latlot[i].1)
                        / (self.latlot[i + 1].0 - self.latlot[i].0))
                        * (x - self.latlot[i].0))
                        .roundf(1000_000);
                    //println!("y {y}");
                    y = (y - self.latlot[i].1).abs();
                    self.rome.push((x, y));
                }
            }
        }
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

    pub fn prediction(&mut self) {
        //let a = self.nowpotion_history.last().unwrap();
        let a = self.nowpotion.unwrap();
        let b = self.nowpotion_history.last().unwrap();
        //self.nowpotion_history.last_mut();
        //self.fm_azimuth(now_postion)

        if self.is_fix != None {
            if self.is_fix.unwrap() {
                let distance = self.distance(&a, b);
                let azimuth = self.azimuth(&a, b);

                self.m_ms = distance / self.gps_msec as f64;

                self.guess_position;
            } else {
                self.guess_position;
            }
        }
    }
}
