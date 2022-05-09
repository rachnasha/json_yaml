use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    // match json_yaml::from_file(&file) {
    //    // Ok(read_string) => println!("The read String {}", read_string),
    //     Err(e) => println!("Error parsing json {}", e),
    // }

    if let Err(e) = json_yaml::from_file(&file){
        println!("Error parsing json {}", e);
    }else{
        println!("Done parsing file");
    }

}
