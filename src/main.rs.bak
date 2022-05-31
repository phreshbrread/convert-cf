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

    // All I know is that this catches args[1] if it is not a number
    if let Err(e) = f64::from_str(args[1].trim()) {
        invalid_input();
        return;
    }

    let num_convert_from: f32 = args[1].trim().parse().unwrap();
    let metric_convert_to = args[2].to_uppercase();

    // Make sure either C or F was specified, otherwise print message and end
    if metric_convert_to == "C" {
        let celsius = (num_convert_from - 32.0) * 5.0 / 9.0;
        println!("{}{}F = {}{}C", num_convert_from, DEGREE, celsius, DEGREE);
    } else if metric_convert_to == "F" {
        let fahren = num_convert_from * 9.0 / 5.0 + 32.0;
        println!("{}{}C = {}{}F", num_convert_from, DEGREE, fahren, DEGREE);
    } else {
        invalid_input();
        return;
    }
}

fn invalid_input() {
    println!("Usage: convert-cf [value to convert from] [C/F (to convert to)]");
}
