use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    if let Err(e) = json_yaml::from_file(&file) {
        println!("Error parsing json {}", e);
    }
}
