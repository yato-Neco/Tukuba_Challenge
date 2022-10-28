

mod xtools;

use xtools::time_sleep;

/// 時間更新
struct SLAM {

    vec:Box<(Vec<[f64;2]>,Vec<[u16;2]>)>,
}


impl SLAM {
    fn new(max:usize) -> Self {
        


        SLAM { vec: Box::new((Vec::with_capacity(max),Vec::with_capacity(max))),}


    }

    fn push(&mut self) {



    }

    fn prtest(&self){

        println!("{:?}",self.vec);

    }

    
}

#[cfg(test)]
mod tests {
    use crate::{SLAM, xtools::time_sleep};

    #[test]
    fn it_works() {
        let mut tmp = SLAM::new(10);
        tmp.vec.0.push([1.0,1.0]);
        loop{

            //tmp.vec.0.push([1.0,1.0]);

            tmp.prtest();


            time_sleep(0,500);
        }
    }


}
