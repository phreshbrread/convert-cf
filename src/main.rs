use std::env;

static DEGREE: char = '°';

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_convert_from: f32 = args[1].trim().parse().unwrap();
    let metric_convert_to = &args[2].to_uppercase();

    if metric_convert_to == "C" {
        let celsius = (num_convert_from - 32.0) * 5.0 / 9.0;
        println!("{}{}F = {}{}C", num_convert_from, DEGREE, celsius, DEGREE);
    } else if metric_convert_to == "F" {
        let fahren = num_convert_from * 9.0 / 5.0 + 32.0;
        println!("{}{}C = {}{}F", num_convert_from, DEGREE, fahren, DEGREE);
    } else {
        println!("Invalid input");
    }
}
