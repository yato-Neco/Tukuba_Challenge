/// queue system
/// 
/// 
use std::collections::VecDeque;
use crate::xtools::{Benchmark,time_sleep};

#[test]
fn test() {


    


    let mut t:Vec<u32>= Vec::<u32>::with_capacity(2);
    t.push(0xfffff);
    println!("{:?}", t.qget());
    println!("{:?}", t);

}

#[derive(Debug)]
struct Queue {

    pub q:Vec<u32>,    

}



pub trait  Qu {
    fn qget(&mut self) -> Option<u32>;
}

impl Qu for Vec<u32> {
    fn qget(&mut self) -> Option<u32> {

        let mut q0:Option<u32> = None;
        if self.len() > 0 {
            q0 = Some(self[0]);
            self.remove(0);
        }

        q0
    }
}

