use nav_types::{ENU, WGS84};

#[test]
fn test() {
    let mut nav = Nav::init();
    nav.set_lat_lot((36.064215, 136.221365));
    nav.gps_senser.is_fix = true;

    let mut waypoints = Vec::new();
    waypoints.push((36.064225, 136.221375));

    nav.add_waypoints(waypoints);

    nav.robot_move(0.6819655742780839, 1.431415478609833);

    println!("position {:?}", nav.position);
    nav.is_in_waypoint();

    nav.set_lat_lot((36.064225, 136.221375));
    nav.robot_move(0.6819655742780839, 1.431415478609833);
    println!("position {:?}", nav.position);
    nav.is_in_waypoint();
    nav.is_in_waypoint();

}

#[derive(Debug)]
pub struct Nav {
    pub lat_lon: Option<(f64, f64)>,
    pub position: (f64, f64),
    pub waypoints: Vec<(f64, f64)>,
    pub destination_index: usize,
    pub lat_lon_history: Vec<(f64, f64)>,
    pub start_lat_lot_index:Option<usize>,
    pub r: f64,
    pub is_simulater: bool,
    pub gps_senser: GpsSenser,
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
            lat_lon_history: Vec::new(),
            start_lat_lot_index:None,
            r: 0.00001,
            is_simulater: false,
            gps_senser: GpsSenser {
                is_fix: false,
                lat_lon: None,
                num_sat: None,
            },
        }
    }

    #[inline]
    /// is_fix: true
    pub fn is_in_waypoint(&mut self) -> bool {
        if self.destination_index < self.waypoints.len() {
            let lat0_bottom = self.waypoints[self.destination_index].0 - self.r;
            let lat0_top = self.waypoints[self.destination_index].0 + self.r;

            let lon1_bottom = self.waypoints[self.destination_index].1 - self.r;
            let lon1_top = self.waypoints[self.destination_index].1 + self.r;

            let mut is_in: bool = false;

            if (lat0_bottom <= self.position.0 && self.position.0 <= lat0_top)
                && (lon1_bottom <= self.position.1 && self.position.1 <= lon1_top)
            {
                println!("In!!");
                is_in = true;
            }

            if is_in {
                self.destination_index += 1;    
            }

            return  false;

        }else{
            //println!("exit");
            return  true;
        }
    }

    #[inline]
    /// is_fix: true
    fn waypoint_azimuth_distance(&self) -> (f64, f64) {
        let pos_a = WGS84::from_degrees_and_meters(
            self.waypoints[self.destination_index].0,
            self.waypoints[self.destination_index].1,
            0.0,
        );
        let pos_b =
            WGS84::from_degrees_and_meters(self.lat_lon.unwrap().0, self.lat_lon.unwrap().1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);

        (azimuth, distance)
    }

    #[inline]
    /// is_fix: true
    fn azimuth_distance(&self, a: &(f64, f64), b: &(f64, f64)) -> (f64, f64) {
        let pos_a = WGS84::from_degrees_and_meters(a.0, a.1, 0.0);
        let pos_b = WGS84::from_degrees_and_meters(b.0, b.1, 0.0);
        let distance: f64 = pos_a.distance(&pos_b);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north());

        (azimuth, distance)
    }

    #[inline]
    /// is_fix: true
    pub fn add_waypoints(&mut self, waypoints: Vec<(f64, f64)>) {
        //println!("{:?}", waypoints);
        for waypoints_lat_lot in waypoints.iter() {
            //let distance = self.distance(start_latlot, latlot) * 100.0;
            //let azimuth = self.azimuth(&start_latlot, latlot);

            let (azimuth, distance) =
                self.azimuth_distance(&self.lat_lon_history[0], waypoints_lat_lot);
            //self.azimuth360(&mut azimuth);
            println!("azimuth(sita)_distance: {:?}", (azimuth, distance));

            let x = azimuth.sin() * distance * 100.0;
            let y = azimuth.cos() * distance * 100.0;

            println!("(y,x): {:?}", (y, x));
            println!("azimuth(sita): {}", (x / y));

            self.waypoints.push((y, x));
        }
    }

    #[inline]
    pub fn set_lat_lot(&mut self, lat_lon: (f64, f64)) {
        let last_lat = self.lat_lon_history.last().unwrap().0;
        let last_lot = self.lat_lon_history.last().unwrap().0;

       
        self.lat_lon = Some(lat_lon);

        if last_lat != lat_lon.0 && last_lot != lat_lon.1 {
            self.lat_lon_history.push(lat_lon);
        };
        
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
            self.position.0 += y;
            self.position.1 += x;
        } else {
            let x = azimuth.sin() * speed * 100.0; // / 100000.0;
            let y = azimuth.cos() * speed * 100.0; // / 100000.0;
            println!("(y,x): {:?}", (y, x));

            //TODO: 代入
            self.position.0+=x;
            self.position.1+=y;
        }
    }

    #[inline]
    pub fn frist_calculate_azimuth(&self) -> f64 {
        
        let nowpotion = self.lat_lon.unwrap();
        let pos_a = WGS84::from_degrees_and_meters(
            self.lat_lon_history[0].0,
            self.lat_lon_history[0].1,
            0.0,
        );
        let pos_b = WGS84::from_degrees_and_meters(nowpotion.0, nowpotion.1, 0.0);
        let vec = pos_b - pos_a;
        let azimuth = f64::atan2(vec.east(), vec.north()) * (180.0 / std::f64::consts::PI);
        azimuth
    }

    

    

}
