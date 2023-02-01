use nav_types::{ENU, WGS84};

#[test]
fn test() {
    let mut nav = Nav::init();
    nav.set_lat_lot((36.064225, 136.221375));
    nav.gps_senser.is_fix = true;

    nav.bkw_azimuth();
    //let mut waypoints = Vec::new();
    //waypoints.push((36.064225, 136.221375));
    //waypoints.push((36.064225, 136.221375));
    //waypoints.push((36.064225, 136.221375));
    //waypoints.push((36.064225, 136.221375));
    //waypoints.push((36.064225, 136.221375));

    //nav.add_waypoints(waypoints);

    //nav.robot_move(0.0, 0.0);

    //println!("position {:?}", nav.position);
    //nav.in_waypoint();

    //nav.set_lat_lot((36.064225, 136.221375));
    //nav.robot_move(0.6819655742780839, 1.431415478609833);
    //println!("position {:?}", nav.position);
    //nav.in_waypoint();
    //nav.in_waypoint();
}

#[derive(Debug)]
pub struct Nav {
    pub lat_lon: Option<(f64, f64)>,
    pub position: (f64, f64),
    pub row_waypoints: Vec<(f64, f64)>,
    pub waypoints: Vec<(f64, f64)>,
    pub destination_index: usize,
    pub lat_lon_history: Vec<(f64, f64)>,
    pub start_lat_lot_index: Option<usize>,
    pub next_azimuth: f64,
    pub start_azimuth: f64,
    pub r: f64,
    pub is_simulater: bool,
    pub gps_senser: GpsSenser,
    pub start_index: Option<usize>,
    pub is_in_waypoint: bool,
}

#[derive(Debug)]
pub struct GpsSenser {
    pub is_fix: bool,
    pub lat_lon: Option<(f64, f64)>,
    pub num_sat: Option<usize>,
    //pub nowtime: String,
}

impl GpsSenser {
    #[inline]
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

        //self.nowtime = gps_format.get(0).unwrap_or(&"").to_string();

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
                if *e == "Fix" {
                    self.is_fix = true;
                } else {
                    self.is_fix = false;
                }
            }
            None => {}
        };

        match gps_format.get(3) {
            Some(lat) => {
                let lot = gps_format.get(4).unwrap();
                self.lat_lon = Some((lat.parse::<f64>().unwrap(), lot.parse::<f64>().unwrap()));
            }
            None => {}
        }
    }
}

impl Nav {
    #[inline]
    /// is_fix: false
    pub fn init() -> Self {
        Self {
            lat_lon: None,
            position: (0.0, 0.0),
            waypoints: Vec::new(),
            destination_index: 0,
            row_waypoints: Vec::new(),
            lat_lon_history: Vec::new(),
            start_lat_lot_index: None,
            r: 11.0,
            is_simulater: false,
            next_azimuth: 0.0,
            start_azimuth: 0.0,
            is_in_waypoint: false,
            gps_senser: GpsSenser {
                is_fix: false,
                lat_lon: None,
                num_sat: None,
            },
            start_index: None,
        }
    }

    #[inline]
    /// is_fix: true
    pub fn in_waypoint(&mut self) -> (bool, bool) {
        if self.destination_index < self.waypoints.len() {
            let area = (
                self.waypoints[self.destination_index].0 - self.r,
                self.waypoints[self.destination_index].0 + self.r,
                self.waypoints[self.destination_index].1 - self.r,
                self.waypoints[self.destination_index].1 + self.r,
            );

            //println!("{:?}", self.waypoints[self.destination_index]);
            //println!("{:?}", self.position);

            if (area.0 <= self.position.0 && self.position.0 <= area.1)
                && (area.2 <= self.position.1 && self.position.1 <= area.3)
            {
                println!("In!!");
                self.destination_index += 1;
                self.is_in_waypoint = true;

                if self.destination_index >= self.waypoints.len() {
                    //println!("{:?}", self.waypoints[self.destination_index]);
                    println!("{:?}", self.position);
                    println!("exit");
                    return (false, true);
                }

                return (true, false);
            }

            self.is_in_waypoint = false;

            return (false, false);
        } else {
            return (false, true);
        }
    }

    #[inline]
    /// is_fix: true
    pub fn waypoint_azimuth_distance(&mut self) -> f64 {
        let pos_a = WGS84::from_degrees_and_meters(
            self.row_waypoints[self.destination_index].0,
            self.row_waypoints[self.destination_index].1,
            0.0,
        );
        let pos_b =
            WGS84::from_degrees_and_meters(self.lat_lon.unwrap().0, self.lat_lon.unwrap().1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);
        let vec = pos_b - pos_a;
        self.next_azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);
        distance
    }

    #[inline]
    /// is_fix: true
    fn azimuth_distance(&self, a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
        /// distance * 100 の影響で角度がおかしい？
        let pos_a = WGS84::from_degrees_and_meters(a.0, a.1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north());

        (azimuth, distance)
    }

    #[inline]
    pub fn set_start_index(&mut self) {
        self.start_index = Some(self.lat_lon_history.len() - 1);
    }

    #[inline]
    pub fn get_start_index(&mut self) -> usize {
        self.start_index.unwrap_or(0)
    }

    #[inline]
    /// is_fix: true
    pub fn add_waypoints(&mut self, waypoints: Vec<(f64, f64)>) {
        //println!("{:?}", waypoints);
        self.row_waypoints = waypoints.clone();

        for waypoints_lat_lot in waypoints.iter() {
            //let distance = self.distance(start_latlot, latlot) * 100.0;
            //let azimuth = self.azimuth(&start_latlot, latlot);

            let (azimuth, distance) =
                self.azimuth_distance(&self.lat_lon_history[0], waypoints_lat_lot);
            //self.azimuth360(&mut azimuth);
            //println!("azimuth {} distance: {}", azimuth, distance);

            let x = azimuth.sin() * distance * 100.0;
            let y = azimuth.cos() * distance * 100.0;

            //println!("(y,x): {:?}", (y, x));
            //println!("azimuth: {}", ((x / y)));

            self.waypoints.push((y, x));
        }
    }

    #[inline]
    pub fn set_lat_lot(&mut self, lat_lon: (f64, f64)) {
        self.lat_lon = Some(lat_lon);

        match self.lat_lon_history.last() {
            Some((lat, lot)) => {
                if *lat != lat_lon.0 || *lot != lat_lon.1 {
                    self.lat_lon_history.push(lat_lon);
                };
            }
            None => {
                self.lat_lon_history.push(lat_lon);
            }
        }
    }

    /// is_fix: false or true
    /// speed: cm
    /// is_fixで分けるべき？
    #[inline]
    pub fn robot_move(&mut self, azimuth: f64, speed: f64) {
        //TODO: retrun azimuth
        if self.gps_senser.is_fix {
            let lat_lon = self.lat_lon.unwrap();

            let (azimuth, distance) = self.azimuth_distance(&self.lat_lon_history[0], &lat_lon);

            let x = azimuth.sin() * distance * 100.0;
            let y = azimuth.cos() * distance * 100.0;
            self.position.0 = y;
            self.position.1 = x;
        } else {
            let x = azimuth.sin() * speed * 100.0; // / 100000.0;
            let y = azimuth.cos() * speed * 100.0; // / 100000.0;
            //println!("(y,x): {:?}", (y, x));

            //TODO: 代入
            self.position.0 += x;
            self.position.1 += y;
        }
    }

    #[inline]
    pub fn frist_calculate_azimuth(&mut self) {
        let nowpotion = self.lat_lon.unwrap();
        let pos_a = WGS84::from_degrees_and_meters(
            self.lat_lon_history[0].0,
            self.lat_lon_history[0].1,
            0.0,
        );
        let pos_b = WGS84::from_degrees_and_meters(nowpotion.0, nowpotion.1, 0.0);
        let vec = pos_b - pos_a;
        self.start_azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);
    }

    pub fn correction(&mut self) {
        //self.azimuth_distance(, );
        self.waypoint_azimuth_distance();
    }

    /// is:fix
    #[inline]
    fn bkw_azimuth(&mut self) -> (f64, f64) {
        //let tmp = self.azimuth_distance( &(35.62616455678764,139.34219715172813),&(35.632018133236116,139.33117493228036));
        let tmp = self.waypoint_azimuth_distance();
        //println!("{:?}", tmp);

        if tmp > 0.0 {
            let az = tmp - 90.0;
            println!("{:?}", az);
            println!("{:?}", az + 180.0);
            return (az, az + 180.0);
        } else {
            let az = tmp + 90.0;
            println!("{:?}", az);
            println!("{:?}", az - 180.0);
            return (az, az - 180.0);
        }
    }

    #[inline]
    fn heron(a: f64, b: f64, c: f64) -> f64 {
        let s = 0.5 * (a + b + c);

        let large_s = (s * (s - a) * (s - b) * (s - c)).sqrt();

        let h = 2.0 * large_s / a;

        h
    }
    

    fn hosei(&mut self) {


        if  self.destination_index <= 0 {
            
            let next_prev = self.azimuth_distance(&self.row_waypoints[self.destination_index],&self.lat_lon_history[self.start_index.unwrap()]);
            let next_now = self.azimuth_distance(&self.row_waypoints[self.destination_index],&self.lat_lon.unwrap());
            let now_prev = self.azimuth_distance(&self.lat_lon.unwrap(), &self.lat_lon_history[self.start_index.unwrap()]);

            
            
        }else {


            
            let next_prev = self.azimuth_distance(&self.row_waypoints[self.destination_index],&self.row_waypoints[self.destination_index - 1]);
            let next_now = self.azimuth_distance(&self.row_waypoints[self.destination_index],&self.lat_lon.unwrap());
            let now_prev = self.azimuth_distance(&self.lat_lon.unwrap(), &self.row_waypoints[self.destination_index - 1]);
            
            
        }
    }

    #[inline]
    fn course(self, h: f64, hdg: f64) -> isize {

        if h > 1.0 {
            if hdg >= 10.0 {
                1
            } else if hdg <= -10.0 {
                -1
            } else {
                0
            }
        } else {
            0
        }
    }



}

pub trait az {
    fn sita_deg(&self) -> f64;
    fn sita_deg_360(&self) -> f64;
}

impl az for f64 {
    fn sita_deg(&self) -> f64 {
        self * (180.0 / std::f64::consts::PI)
    }

    fn sita_deg_360(&self) -> f64 {
        self * (180.0 / std::f64::consts::PI)
    }
}
