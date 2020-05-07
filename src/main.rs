extern crate toml_reader;
extern crate toml;

use std::fs::File;
use std::io::Read;
use toml_reader::structs::config::*;

fn main() {
    parse("C:\\Users\\Beni Reydman\\Documents\\Work\\Rust Code\\toml_reader\\files\\config.toml".to_string());
}

/***
* Function Parse:
*
* Purpose:
* to read toml files and return a struct consisting of
* a map of sensors and modbus's
***/
pub fn parse (path: String) -> Config {
    /* Get toml file */
    let toml_file = read_file(path);

    let config: Config = toml::from_str(&toml_file).unwrap();
    println!("{:?}", config);
    return config;
}

/***
* Function read_file:
*
* Purpose:
* Read toml file and return read String 
***/
pub fn read_file(path: String) -> String {
    /* Variables */
    let mut toml_file = String::new();

    /* Attempting to open file */
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_)  => {
            println!("Could not find config file, returning empty string.");
            return String::new();
        }
    };

    /* Attempt to read in the file */
    file.read_to_string(&mut toml_file)
                .unwrap_or_else(|err| panic!("Error reading file: {0}", err));

    /* Give a warning if file is empty (Not necessarily a bug, will return empty Config) */
    if toml_file.is_empty() {
        println!("Warning, empty file!");
    }

    return toml_file;
}
