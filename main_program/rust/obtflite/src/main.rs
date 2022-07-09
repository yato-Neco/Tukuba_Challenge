use std::fs::{self, File};
use std::io::Read;

use tflite::ops::builtin::BuiltinOpResolver;
use tflite::{FlatBufferModel, InterpreterBuilder, Result, model};



fn main() {
    let model = Model::from_file("data/MNISTnet_uint8_quant.tflite").unwrap();

    println!("Hello, world!");
}
