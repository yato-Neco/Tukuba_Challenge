extern crate flacon;
extern crate gps;
extern crate lidar;
extern crate load_shdlib;
extern crate slam;
extern crate rthred;

mod robot;
mod xtools;
use clap::Parser;
use robot::mode::Mode;

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
        "auto" => Mode::auto(),
        "key" => Mode::key(),
        "test" => Mode::test(),
        "display" => {}
        "k" => Mode::key(),
        "a" => Mode::auto(),
        "d" => {}
        _ => {}
    }
}
