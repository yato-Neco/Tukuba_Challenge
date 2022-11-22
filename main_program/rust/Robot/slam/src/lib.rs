
use std::collections::HashMap;
use mytools::time_sleep;



/// 時間更新
pub struct SLAM {
    // (robot_postion[x,y], lider_data [frequency,distance])
    vec: Vec<((f64, f64), Vec<(f64, f64)>)>,
    max: usize,
}

impl SLAM {
    pub fn new(max: usize) -> Self {
        Self {
            vec: Vec::with_capacity(max),
            max,
        }
    }

    fn load_point(&mut self) {

    }

    pub fn push(&mut self, value: ((f64, f64), Vec<(f64, f64)>)) {
        self.vec.push(value);
        self.del();
    }

    
    fn get(&self)  {
        //(self.vec.0.get(self.max - 1), self.vec.1.get(self.max - 1))
    }

    pub fn obb(&self){

        let slam_data = self.vec.get(0).unwrap();

        let lidar_data = slam_data.1.get(0).unwrap();

        for p in slam_data.1.iter() {
            
            if 175.0 > p.0 && p.0 > 185.0 &&  p.1 < 3.0 {
                
                
                
            }   
                
        }       

        //&self.vec.get(0).unwrap().1.get(0).unwrap();
    }
    

    pub fn del(&mut self) {
        if self.vec.len() > self.max {
            self.vec.remove(0);
        }
    }

    fn prtest(&self) {
        println!("{:?}", self.vec);
    }
}

#[cfg(test)]
mod tests {
    use crate::{time_sleep, SLAM};

    #[test]
    fn it_works() {
        let mut tmp = SLAM::new(3);

        let mut count: f64 = 0.0;
        loop {
            //tmp.vec.0.push([1.0,1.0]);

            println!("{:?}", tmp.vec);
            //println!("{:?}", tmp.get());
            time_sleep(0, 500);

            count += 1.0;
        }
    }
    #[test]
    fn for_test() {
        for i in (0..100).step_by(2) {
            println!("{}",i);
        }
    }
}
