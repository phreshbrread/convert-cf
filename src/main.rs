use std::env;
use std::str::FromStr;

static DEGREE: char = '°';

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // If there are not 2 arguments, print message and end
    if args.len() != 3 {
        invalid_input();
        return;
    }

    // Try parse args[1], if it causes a ParseFloatError,
    // go to invalid_input(), otherwise continue like normal
    if let Err(_e) = f64::from_str(args[1].trim()) {
        invalid_input();
        return;
    }

    // TODO check if value is C or F before calculate

    // TODO print final result properly
    println!(
        "{}",
        calculate(args[1].trim().parse().unwrap(), args[2].to_uppercase())
    );
}

fn calculate(value_to_convert_from: f32, metric_to_convert_to: String) -> f32 {
    match metric_to_convert_to.as_str() {
        "C" => return (value_to_convert_from - 32.0) * 5.0 / 9.0,
        "F" => return value_to_convert_from * 9.0 / 5.0 + 32.0,
        _ => invalid_input(),
    };
}

fn invalid_input() {
    println!("Usage: convert-cf [value to convert from] [C/F (to convert to)]");
}
