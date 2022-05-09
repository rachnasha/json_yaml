use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

/// read the whole file in a string and parse the string to yaml ... not very efficient for mem.
pub fn json_to_yaml_string(file: &String) -> Result<(), Box<dyn Error>> {
    let s = fs::read_to_string(file)?;
   // println!("The read file: {:?}", s);
    let v: Value = serde_json::from_str(&s)?;
    //println!("the value json {:?}", v);
    let yaml_str = serde_yaml::to_string(&v)?;
    println!("The yaml {}", yaml_str);
    Ok(())
}


/// use a buffered reader ....slightly more efficient depending on serde's functions efficiency.
pub fn from_file(file_name: &String) -> Result<(), Box<dyn Error>> {
    let file_handle = File::open(file_name)?;
    let  br: BufReader<File> = BufReader::new(file_handle);
    to_yaml(&to_json(br)?)?;
   Ok(())

    // for this_line in file_lines {
    //     if let Ok(read_line) = this_line {
    //         println!("The read line {}", read_line);
    //         let v = to_json(&read_line);
    //         println!("the value json {:?}", v);
    //     }
    // };
    // let v: Value = serde_json::from_str(&s)?;
    // println!("the value json {:?}", v);
    // //let json_str = v.to_string();
    // // println!("the value str converted {}", json_str);
    // let yaml_str = serde_yaml::to_string(&v)?;
    // println!("The yaml {}", yaml_str);
    // Ok(yaml_str)
}


fn to_json(br: BufReader<File>) -> Result<Value, Box<dyn Error>> {
    let v: Value = serde_json::from_reader(br)?;
    Ok(v)
}


fn to_yaml(value: &Value) -> Result<(), Box<dyn Error>>{
    let yaml_str = serde_yaml::to_string(&value)?;
    println!("The yaml {}", yaml_str);
    Ok(())
}