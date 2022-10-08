mod xtools;

use xtools::time_sleep;

/// 時間更新
struct SLAM {
    // (robot_postion[x,y], lider_data [frequency,distance])
    vec: Box<(Vec<[f64; 2]>, Vec<[u16; 2]>)>,
    max: usize,
}

impl SLAM {
    fn new(max: usize) -> Self {
        Self {
            vec: Box::new((Vec::with_capacity(max), Vec::with_capacity(max))),
            max,
        }
    }

    fn push(&mut self) {}

    fn get(&self) {
        
        self.vec.0.get(0);
        self.vec.1.get(0);


    }
    fn del(&mut self) {
        if self.vec.0.len() > self.max {
            self.vec.0.remove(0);
        }
    }

    fn prtest(&self) {
        println!("{:?}", self.vec);
    }
}

#[cfg(test)]
mod tests {
    use crate::{xtools::time_sleep, SLAM};

    #[test]
    fn it_works() {
        let mut tmp = SLAM::new(10);
        tmp.vec.0.push([1.0, 1.0]);
        loop {
            //tmp.vec.0.push([1.0,1.0]);

            tmp.prtest();

            time_sleep(0, 500);
        }
    }
}
