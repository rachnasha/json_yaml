use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

/// read the whole file in a string and parse the string to yaml ... not very efficient for mem.
pub fn json_to_yaml_string(file: &str) -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string(file)?;
    // println!("The read file: {:?}", s);
    let v: Value = serde_json::from_str(&s)?;
    //println!("the value json {:?}", v);
    let yaml_str = serde_yaml::to_string(&v)?;
    println!("{}", yaml_str);
    Ok(())
}

/// use a buffered reader ....slightly more efficient depending on serde's functions efficiency.
pub fn from_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file_handle = File::open(file_name)?;
    let br: BufReader<File> = BufReader::new(file_handle);
    let value: Value = serde_json::from_reader(br)?;
    let yaml_str = serde_yaml::to_string(&value)?;
    println!("{}", yaml_str);
    Ok(())
}
