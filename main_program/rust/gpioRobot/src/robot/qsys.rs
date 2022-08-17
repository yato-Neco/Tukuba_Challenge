/// queue system
/// 
/// 

#[test]
fn test() {

    let mut a = Ms::Time(9.0);

    println!("{:?}",a);


}

#[derive(Debug)]
enum Ms {

    Stop,
    Start,
    Time(f32)
    
}


struct M {

    ms:Ms,
    lv:usize

}

struct Queue {

    

}