use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use std::env;
use std::fs::{self, File};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct ProgamData {
    id: String,
    info: Info,
}


#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    tags: Vec<String>,
    author: String,
    descrption: String,
    reference: Vec<String>,
}

fn read_yaml_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if let Some(program_name) = args.get(0) {
        println!("Program name:{}", program_name);
    }

    for (index, arg) in args.iter().skip(1).enumerate() {
        println!("Argument {}: {}", index + 1, arg);
    }

    let search_value = &args[1];

    let dir_path = "config";

    match fs::read_dir(dir_path){
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "yaml" {
                            match read_yaml_file(path.to_str().unwrap()){
                                Ok(yaml_string) =>{
                                    let data: ProgamData =
                                        serde_yaml::from_str(&yaml_string).expect("Failed to deserialize YAML");
                                    if &data.id == search_value {
                                        println!("{:#?}", data);
                                    }
                                }
                                Err(err) => {
                                    eprintln!("Failed to read file: {}", err);
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to read directory: {}", err);
        }
    }
}
