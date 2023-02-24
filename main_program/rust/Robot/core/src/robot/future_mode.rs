use mytools::time_sleep;
use scheduler::Scheduler;
#[derive(Debug, Clone, PartialEq)]
enum State {
    Waiting = 0,
    Running = 1,
    Pending = 2,
}

#[derive(Debug, Clone)]
struct Instruction {
    index: usize,
    priority: u8,
    state: State,
    pub moter_order: u32,
    pub latlot: Option<(f32, f32)>,
    block_time: i128,
}
#[derive()]
struct Instructions {
    vec: Vec<Instruction>,
    is_locked: bool,
    block_time: i128,
    scheduler: Scheduler,
}

impl Instructions {
    fn new() -> Self {
        Self {
            vec: Vec::new(),
            is_locked: false,
            block_time: 0,
            scheduler: Scheduler::start(),
        }
    }

    fn add(&mut self, priority: u8, order: u32, time: i128) {
        self.vec.push(Instruction {
            index: self.vec.len(),
            priority: priority,
            state: State::Waiting,
            moter_order: order,
            latlot: None,
            block_time: time,
        });
    }

    fn execution(&mut self) {
        let len = self.vec.len();

        if len > 0 {
            let index = len - 1;
            //println!("{:?}",self.vec.iter().find(|x| x.priority == 0));

            if self.vec[index].priority == 0 {}

            if self.vec[index].state == State::Waiting {
                if self.vec[index].priority == 0 {}

                self.block_time += self.vec[index].block_time;
                self.vec[index].state = State::Running;
                println!("{:?}", self.vec[index]);
            }

            //println!("{}",self.block_time);

            if self.scheduler.nowtime() >= self.block_time {
                self.vec.remove(index);
            }
        }

        //println!("{:?}",self.vec.iter().find(|x| x.priority == 0));
    }
}

#[test]
fn auto() {
    let mut tmp = Instructions::new();

    tmp.add(1, 2, 3000);

    tmp.add(0, 3, 2000);

    tmp.add(1, 2, 3000);

    let mut count = 0;
    loop {
        tmp.execution();
        time_sleep(0, 500);
        //println!("{}",start_time.nowtime());
        if count == 2 {
            tmp.add(0,3,2000);
        }
        count += 1;
    }
}
