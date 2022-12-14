extern crate flacon;
extern crate gps;
extern crate lidar;
extern crate load_shdlib;
extern crate slam;
extern crate rthred;
extern crate scheduler;
extern crate mytools;
extern crate robot_serialport;


mod robot;
use clap::Parser;
use robot::mode::{key::key,test::test as other_test,auto::auto};


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    mode: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.mode.as_str() {
        "auto" => auto(),
        "key" => key(),
        "test" => other_test(),
        "k" => key(),
        "a" => auto(),
        _ => {}
    }
}


#[test]
fn test() {
   

    //println!("{}", (((i128::MAX / 1000) / 120) / 24) / 365);
}