use mytools::{time_sleep, Xtools};
use nav_types::{ENU, WGS84};
use rthred::sendG;
use std::sync::mpsc::Sender;
use std::time::Duration;

#[test]
fn test() {
    let mut tmp = GPS::new(false);
    let waypoints = vec![(35.631317, 139.330912)];
    //let start_latlot = (35.631316, 139.330911);
    //35.672654,139.746093
    //35.631316, 139.330912

    //35.627471, 139.340386
    //35.623271, 139.345627
    let now_latlot = (35.627471, 139.340386);
    tmp.nowpotion_history = vec![(35.627471, 139.340386)];
    tmp.waypoints = vec![(35.627469, 139.340383)];


    println!("{:?}",tmp.azimuth360(tmp.azimuth(&now_latlot, &waypoints[0])));

    tmp.generate_rome();
    println!("{:?}",tmp.rome.now_index);
    println!("{:?}",tmp.rome.index_order);
    
    for i in tmp.rome.mesh_map.iter() {
        println!("{:?}", i);
    }
    
    
    println!("{}", "-".repeat(80));
    
    println!("{}",tmp.rome.next_point_azimuth());
    tmp.rome.set_azimuth(tmp.rome.next_point_azimuth());
    //tmp.azimuth_string(0.0);
    tmp.rome.robot_move(3.0);

    for i in tmp.rome.mesh_map.iter() {
        println!("{:?}", i);
    }
    println!("{}", "-".repeat(80));
    
    println!("{:?}",tmp.rome.mesh_map.len());
    
    
    /*
    
    //is fix 以降。
    tmp.generate_rome(waypoints);
    //tmp.rome.set_azimuth(azimuth);
    println!("{:?} : {:?}", tmp.rome.azimuth, tmp.rome.now_index);

    for i in tmp.rome.mesh_map.iter() {
        println!("{:?}", i);
    }
    println!("{}", "-".repeat(80));

    tmp.rome.gps_robot_move(&now_latlot);
    //tmp.rome.set_azimuth(azimuth);
    println!("{:?} : {:?}", tmp.rome.azimuth, tmp.rome.now_index);

    for i in tmp.rome.mesh_map.iter() {
        println!("{:?}", i);
    }

    println!("{}", "-".repeat(80));

    tmp.rome.gps_robot_move(&(35.631317, 139.330912));
    //tmp.rome.set_azimuth(azimuth);
    println!("{:?} : {:?}", tmp.rome.azimuth, tmp.rome.now_index);

    for i in tmp.rome.mesh_map.iter() {
        println!("{:?}", i);
    }
    //println!("{:?}",tmp.azimuth_none_gps(lat,lot));
    */
    
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
    pub waypoints: Vec<(f64, f64)>,
    pub in_waypoint: bool,
    pub next_latlot: Option<(f64, f64)>,
    pub is_simulater: bool,
    pub _rome: Vec<(f64, f64)>,
    pub is_nowpotion_history_sub: bool,
    pub nowtime: String,
    pub gps_format: Vec<String>,
    pub guess_position: Option<(f64, f64)>,
    pub gps_msec: u32,
    pub m_ms: f64,
    pub rome: Rome,
    pub wt901: WT901,
}

#[derive(Debug, Clone)]
pub struct WT901 {
    pub ang: Option<(f32, f32, f32)>,
    pub mag: Option<(u32, u32, u32)>,
}

#[derive(Debug, Clone)]
pub struct Rome {
    pub azimuth: f64,
    pub mesh_map: Vec<Vec<u8>>,
    pub robot_start_index: usize,
    pub now_index: (usize, usize),
    pub start_latlot: Option<(f64, f64)>,
    pub index_order: Vec<(usize, usize)>,
}

impl Rome {

    #[inline]
    fn senser_init(&mut self) {


        todo!()
    }


    /// set robot_start_index (x and y: range / 2) and mesh_map (generation 2 dimensional array)
    /// range / 1cm
    #[inline]
    fn mesh_generation(&mut self, range: usize) {
        self.set_start_index(&range);
        println!("{}",range);
        for _ in 0..=range {
            let mut mesh_map_x = Vec::with_capacity(range);
            for _ in 0..=range {
                mesh_map_x.push(0);
            }
            self.mesh_map.push(mesh_map_x);
        }

        self.mesh_map[self.robot_start_index][self.robot_start_index] = 1;
    }

    /// mesh_mapにwaypointsを追加
    /// waypointsの表現は2
    #[inline]
    fn add_waypoints(&mut self, waypoints: &Vec<(f64, f64)>, start_latlot: &(f64, f64)) {
        //println!("{:?}", waypoints);
        for latlot in waypoints.iter() {
            let distance = self.distance(start_latlot, latlot) * 100.0;
            let azimuth = self.azimuth(&start_latlot, latlot);

            //TODO: x,yが反転してる気がする。
            let x = (azimuth.sin() * distance).round();
            let y = (azimuth.cos() * distance).round();
            println!("{:?}",(y,x));
            if x != 0.0 || y != 0.0 {
                let y_index = (self.robot_start_index as f64 - (y / 10.0).round()) as usize;
                let x_index = (self.robot_start_index as f64 + (x / 10.0).round() ) as usize;
                self.mesh_map[y_index][x_index] = 2;
                self.index_order.push((y_index,x_index));
            }
        }
    }

    /// GPSありきの位置
    #[inline]
    fn gps_robot_move(&mut self, now_latlot: &(f64, f64)) {
        self.mesh_map[self.now_index.0][self.now_index.1] = 0;

        let distance = self.distance(&self.start_latlot.unwrap(), &now_latlot) * 100.0;
        let azimuth = self.azimuth(&self.start_latlot.unwrap(), &now_latlot);

        let x = (azimuth.sin() * distance).round() as usize;
        let y = (azimuth.cos() * distance).round() as usize;

        self.mesh_map[self.robot_start_index - y][self.robot_start_index + x] = 1;

        self.now_index = (self.robot_start_index - y, self.robot_start_index + x);
    }

    /// GPSなしの位置
    /// is not fix の時
    #[inline]
    fn non_gps_robot_move(&mut self, speed:f64) {
        //let distance = self.distance(last_latlot, b);
        let azimuth = (self.azimuth - 180.0) * -1.0;


        let azimuth = azimuth * (std::f64::consts::PI / 180.0);

        //let azimuth = (self.azimuth -180.0).abs() * (std::f64::consts::PI / 180.0);
        
        
        let x = self.now_index.1 as f64 + (azimuth.sin() * speed);
        let y = self.now_index.0 as f64 + (azimuth.cos() * speed);
        //speed;

        println!("{:?}",(y,x));

        self.mesh_map[self.now_index.0][self.now_index.1] = 0;
        self.now_index = (y as usize,x as usize);
        self.mesh_map[self.now_index.0][self.now_index.1] = 1;
        
        
    }

    #[inline]
    fn set_start_index(&mut self, range: &usize) {
        self.robot_start_index = range / 2;
        self.now_index = (self.robot_start_index, self.robot_start_index);
    }


    pub fn set_azimuth(&mut self, azimuth: f64) {
        println!("{}",azimuth);
        self.azimuth = azimuth;
    }

    /// a,b 距離
    #[inline]
    fn distance(&self, a: &(f64, f64), b: &(f64, f64)) -> f64 {
        //let nowpotion = self.nowpotion.unwrap();
        let pos_a = WGS84::from_degrees_and_meters(a.0, a.1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);

        distance
    }

    /// a,bの角度
    #[inline]
    pub fn azimuth(&self, a: &(f64, f64), b: &(f64, f64)) -> f64 {
        let pos_a = WGS84::from_degrees_and_meters(a.0, a.1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north());

        azimuth
    }


    fn none_gps_azimuth(&self,a:&(usize,usize),b:&(usize,usize)) -> f64{


        let y = a.0  as f64  - b.0 as f64;
        let x = b.1 as f64 - a.1 as f64;

        let tansi = y / x ;

        println!("x y : {:?}",(y,x));
        //println!("{:?}",tansi);
        //println!("{:?}",(tansi.tan() * 180.0 / std::f64::consts::PI / 2.0).round());

        if y < 0.0 && x == 0.0 {
            return 0.0;
        }else if y == 0.0 && x > 0.0 {
            return  90.0;
        }else if y > 0.0 && x == 0.0{
            return  180.0;
        }else if y == 0.0 && x < 0.0 {
            return  270.0;
        } else if y == 0.0 && x == 0.0 {
            return  0.0;
        }else{
            return ( (tansi.tan() * 180.0 / std::f64::consts::PI /2.0).round()  ) * -1.0
        }

        

        

    }


    fn next_point_azimuth(&self) -> f64{


       let azimuth =  self.none_gps_azimuth(&self.now_index, &self.index_order[0]);
        println!("{}",(azimuth) * 1.0);
        azimuth 
     }


    fn robot_move(&mut self,speed:f64) {

        /*
        let azimuth = if  self.azimuth > 270.0 { 
            (self.azimuth - 360.0).abs() -  180.0

         }else if  self.azimuth > 180.0 { 
            (self.azimuth - 270.0).abs() - 90.0
        }else{
            (self.azimuth - 180.0).abs()
        };
        */
        
        //let azimuth = (self.azimuth - 180.0) * -1.0;


        let azimuth = self.azimuth * (std::f64::consts::PI / 180.0);

        //let azimuth = (self.azimuth -180.0).abs() * (std::f64::consts::PI / 180.0);
        
        
        let x = self.now_index.1 as f64 + (azimuth.sin() * speed);
        let y = self.now_index.0 as f64 + (azimuth.cos() * speed);
        //speed;

        println!("{:?}",(y,x));
        //self.mesh_map.get(0); //TODO:mesh_map外検知
        self.mesh_map[self.now_index.0][self.now_index.1] = 0;
        self.now_index = (y.round() as usize,x.round() as usize);
        self.mesh_map[self.now_index.0][self.now_index.1] = 1;

    }

    


    fn in_waypoint(&mut self) -> bool {

        if self.mesh_map[self.now_index.0][self.now_index.1] == 2{
            return  true;
        }else{
            return  false;
        };
        
    }
    

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
            waypoints: Vec::new(),
            next_latlot: None,
            in_waypoint: false,
            is_simulater: simulater,
            _rome: Vec::with_capacity(100),
            is_nowpotion_history_sub: simulater,
            nowtime: String::new(),
            gps_format: Vec::new(),
            guess_position: None,
            gps_msec: 1000,
            m_ms: 0.0,
            rome: Rome {
                azimuth: 0.0,
                mesh_map: Vec::new(),
                robot_start_index: 0,
                now_index: (0, 0),
                start_latlot: None,
                index_order:Vec::new(),
            },
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

    
    
    /// シリアル通信から来るデータ扱いやすく、パースする。
    #[inline]
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
            if self.waypoints[0].0 > self.nowpotion.unwrap().0 {
                self.nowpotion = Some((
                    (self.nowpotion.unwrap().0 + 0.001).roundf(1000),
                    //roundf(self.nowpotion.unwrap().0 + 0.001, 1000),
                    self.nowpotion.unwrap().1,
                ));
            } else if self.waypoints[0].0 < self.nowpotion.unwrap().0 {
                self.nowpotion = Some((
                    (self.nowpotion.unwrap().0 - 0.001).roundf(1000),
                    self.nowpotion.unwrap().1,
                ));
            }
            if self.waypoints[0].1 > self.nowpotion.unwrap().1 {
                self.nowpotion = Some((
                    self.nowpotion.unwrap().0,
                    (self.nowpotion.unwrap().1 + 0.001).roundf(1000),
                ));
            } else if self.waypoints[0].1 < self.nowpotion.unwrap().1 {
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
        let result = match self.waypoints.len() {
            0 => false,
            1.. => {
                let box_flag: bool = self.r#box(
                    &(self.waypoints[0].0, self.waypoints[0].1),
                    &self.nowpotion.unwrap(),
                    self.r,
                );

                (self.azimuth, self.distance) = self.fm_azimuth(&self.nowpotion.unwrap());

                self.running_simulater(self.is_simulater);

                self.in_waypoint = box_flag;

                if box_flag {
                    self.waypoints.remove(0);
                }

                true
            }
            _ => false,
        };

        result
    }

    /// 二地点間の角度(度数法) と 距離
    #[inline]
    fn fm_azimuth(&self, now_postion: &(f64, f64)) -> (f64, f64) {
        let pos_a = WGS84::from_degrees_and_meters(self.waypoints[0].0, self.waypoints[0].1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(now_postion.0, now_postion.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);

        //println!("{}", distance);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        (azimuth, distance)
    }

    ///  frist 実行時 二地点間の角度(度数法)
    #[inline]
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

    #[inline]
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
    #[inline]
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

    pub fn azimuth_string(&self,azimuth:f64) {
        let azimuth_isize = azimuth.round() as isize;
        let tmp =  match  azimuth_isize {
            -45..=45 => "北",
            46..=135 => "東",
            -135..=-44 => "西",
            _ => "南"
        };

        println!("{}",tmp);

    }


    pub fn azimuth360(&self,azimuth:f64) -> usize {
        let mut azimuth_isize = azimuth.round() as isize;

        if azimuth_isize < 0 {
            azimuth_isize += 360;
        }

        println!("{}",azimuth_isize);

        return azimuth_isize as usize
    }

    #[inline]
    pub fn azimuth_none_gps(&self, a: &(f64, f64), b: &(f64, f64)) -> f64 {
        (b.0 - a.0).atan2(b.1 - a.1)
        //let tmp = ((a.0 * 1000_000.0 - b.0 * 1000_000.0).abs(), (a.1 * 1000_000.0 - b.1 * 1000_000.0).abs());
        //let tmp2 = (0.0,0.0);

        //let ab_vec = ((a.0 * b.0) + (a.1 * b.1)) / ((a.0.powf(2.0) + a.1.powf(2.0)).sqrt() * (b.0.powf(2.0) + b.1.powf(2.0)).sqrt());

        //println!("{:?}",ab_vec);
    }


    #[inline]
    pub fn generate_rome(&mut self) {
        //　一番遠いwaypointsのアルゴリズム -->
        let mut distance_vec = Vec::new();
        const EXPECT_MSG: &str = "nowpotion_history index is 0";
        
        for p in self.waypoints.iter() {
            let pos_a = WGS84::from_degrees_and_meters(
                self.nowpotion_history.get(0).expect(EXPECT_MSG).0,
                self.nowpotion_history.get(0).expect(EXPECT_MSG).1,
                0.0,
            );
            let pos_b = WGS84::from_degrees_and_meters(p.0, p.1, 0.0);
            let distance: f64 = pos_a.distance(&pos_b);
            distance_vec.push(distance);
        }

        distance_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // <--

        // メートル to センチ
        let range = (distance_vec.last().unwrap() * 100.0) as usize * 2;
        // 1cm

        println!("range {}",(range as f32 / 10.0).round());

        self.rome.mesh_generation((range as f32 / 10.0).round() as usize);
        self.rome
            .add_waypoints(&self.waypoints, &self.nowpotion_history.get(0).expect(EXPECT_MSG));
        self.rome.start_latlot = Some(*self.nowpotion_history.get(0).expect(EXPECT_MSG));
    }





    pub fn _generate_rome(&mut self) {
        let len = self.waypoints.len() - 1;

        //println!("{}",self.latlot[0].0 - self.latlot[1].0);

        for i in 0..len {
            let sub0: f64 = (self.waypoints[i].0 - self.waypoints[i + 1].0).roundf(1000_000);
            let sub1: f64 = (self.waypoints[i].1 - self.waypoints[i + 1].1).roundf(1000_000);

            //println!("{:?} : {:?}",self.latlot[i],self.latlot[i + 1]);
            //println!("{}",sub0);

            if sub0.abs() >= 0.000009 || sub1.abs() >= 0.000009 {
                let x = (self.waypoints[i].0 + (sub0 / 2.0)).roundf(1000_000);

                if self.waypoints[i] == self.waypoints[i + 1] {
                    let y = self.waypoints[i].1;
                    self._rome.push((x, y))
                } else {
                    let mut y = (((self.waypoints[i + 1].1 - self.waypoints[i].1)
                        / (self.waypoints[i + 1].0 - self.waypoints[i].0))
                        * (x - self.waypoints[i].0))
                        .roundf(1000_000);
                    //println!("y {y}");
                    y = (y - self.waypoints[i].1).abs();
                    self._rome.push((x, y));
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
