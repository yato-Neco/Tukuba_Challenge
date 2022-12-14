struct IMap<I> {
    data: Vec<IntMap<I>>,
}

#[derive(PartialEq, Debug)]
struct IntMap<I> {
    key: usize,
    value: I,
}

impl<I: std::cmp::PartialEq> IMap<I> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, key: usize, value: I) {
        let tmp = self.fnd(key);
        if tmp == None {
            self.data.push(IntMap { key, value });
        } else {
        }
    }

    fn fnd(&self, key: usize) -> Option<&I> {
        //self.data.iter().find(|d| d.key == key);
        let mut tmp = None;
        self.data.iter().for_each(|d| {
            if d.key == key {
                tmp = Some(d)
            }
        });

        if tmp != None {
            return Some(&tmp.unwrap().value);
        } else {
            None
        }
    }
}

#[test]
fn test() {
    let mut tmp = IMap::new();

    tmp.push(0, "a");

    let r = tmp.fnd(0);
    println!("{:?}", r);
}
