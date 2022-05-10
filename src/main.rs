use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Filename not provided. Please rerun passing json filename as an argument.");
        process::exit(1);
    }

    if let Err(e) = json_yaml::from_file(&args[1]) {
        println!("Error parsing json {}", e);
    }
}
