use std::{env, fs};
use locus::gen::three;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_path = args.get(1).expect("input file not provided");

    let mut output_path = &String::from("output.js");
    let mut target = "three";

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "-o" => {
                output_path = args.get(i+1).expect("output path not provided");
            }

            "-t" => {
                target = args.get(i+1).expect("target type not provided");
            }

            _ => ()
        }
    }

    let source = fs::read_to_string(source_path).expect("invalid input file path");

    if target.eq("three") {
        let result = three(source.as_str());
        fs::write(output_path, result.unwrap());
    }
}
