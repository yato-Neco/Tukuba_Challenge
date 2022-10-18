extern crate flacon;
extern crate load_shdlib;
extern crate slam;
extern crate gps;

mod robot;
mod xtools;
use robot::mode::Mode;
use clap::Parser;


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
        "display" => {}
        "k" => Mode::key(),
        "a" => Mode::auto(),
        "d" => {}
        _ => {}
    }
}

