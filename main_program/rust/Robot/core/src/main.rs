extern crate flacon;
extern crate gps;
extern crate lidar;
extern crate load_shdlib;
extern crate slam;
extern crate rthred;
extern crate scheduler;
extern crate mytools;
extern crate robot_serialport;
extern crate wt901_rs;

mod robot;
use clap::Parser;
use robot::mode::{key::key,test::test as other_test,auto::auto,srpauto,test2};


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    mode: String,
}

fn main() {
    let args = Args::parse();
    match args.mode.as_str() {
        "auto" => auto(),
        "key" => key(),
        "test" => other_test(),
        "k" => key(),
        "a" => auto(),
        "az" => test2::test(),
        _ => {srpauto::auto()}
    }
}


#[test]
fn test() {
   

    //println!("{}", (((i128::MAX / 1000) / 120) / 24) / 365);
}