use std::env;
use std::str::FromStr;

const DEGREE: char = '°';

const INVALID_INPUT: &str = "Usage: convert-cf [value to convert from] [C/F (to convert to)]";

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // If there are not 2 arguments, print message and end
    if args.len() != 3 {
        println!("{}", INVALID_INPUT);
        return;
    }

    // Try parse args[1], if it causes a ParseFloatError,
    // go to invalid_input(), otherwise continue like normal
    if let Err(_e) = f64::from_str(args[1].trim()) {
        println!("{}", INVALID_INPUT);
        return;
    }

    // Check if value is C or F
    if args[2].to_uppercase() == "C" || args[2].to_uppercase() == "F" {
    } else {
        println!("{}", INVALID_INPUT);
        return;
    }

    // TODO print final result properly
    let result: f32 = calculate(args[1].trim().parse().unwrap(), args[2].to_uppercase());

    match args[2].to_uppercase().as_str() {
        "C" => println!("{}{}C = {}{}F", args[1], DEGREE, result, DEGREE), // final c
        "F" => println!("{}{}F = {}{}C", args[1], DEGREE, result, DEGREE), // final f
        _ => println!("{}", INVALID_INPUT),
    };
    return;
}

fn calculate(value_to_convert_from: f32, metric_to_convert_to: String) -> f32 {
    match metric_to_convert_to.as_str() {
        "C" => return (value_to_convert_from - 32.0) * 5.0 / 9.0,
        "F" => return value_to_convert_from * 9.0 / 5.0 + 32.0,
        _ => return 0.0,
    };
}
