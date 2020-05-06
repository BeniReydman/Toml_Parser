extern crate toml_reader;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use toml_reader::structs::config::*;

/* Constants */
const EMPTY: usize = 0;
const MODBUS: usize = 1;
const SENSOR: usize = 2;

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

    /* Setting up config struct */
    let mut new_sensor: HashMap<String, bool> = HashMap::new();
	let mut new_modbus: HashMap<String, ModbusConfig>  = HashMap::new();
    let mut config_toml: Config = Config { sensor: new_sensor, modbus: new_modbus};

    /* Variables */
    let mut curr_type = EMPTY;
    let mut curr_modbus = ModbusConfig::default();  // empty struct
    let mut curr_modbus_name = String::new();

    /* Iterate through file */
    for line in toml_file.lines() {
        /* check to see if there is new type */
        curr_type = check_type(line.to_string(), curr_type);

        /* Parse values based on line  */
        if line.trim().chars().next().unwrap() == '[' {  // check if line starts with [ 
            if !is_empty_modbus(curr_modbus) {  // if struct is not empty, add to map
                new_modbus.insert(curr_modbus_name, curr_modbus);
            }
            if is_new_modbus(line.to_string()) {  // if statement is for [sensor] case
                curr_modbus_name = line.trim().chars().skip(8).collect();  // remove [modbus. from string
                curr_modbus_name.pop();  // remove ] from the end
                println!("{0}", curr_modbus_name.to_string());
                curr_modbus = ModbusConfig::default();  // set as new empty config
            }
        } else if curr_type == MODBUS {
            parse_modbus(line.to_string(), curr_modbus);
        } else if curr_type == SENSOR {
            parse_sensor(line.to_string(), new_sensor);
        }  // else ignore

    }

    return config_toml;
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

/***
* Function check_type:
*
* Purpose:
* Checks to see what type of parsing should be done
* and returns corresponding value
***/
pub fn check_type(line: String, curr: usize) -> usize {
    if line.contains("[modbus]") {
        return MODBUS;
    } else if line.contains("[sensor]") {
        return SENSOR;
    }

    return curr;
}

/***
* Function is_new_modbus:
*
* Purpose:
* Checks if new modbus needs to be parsed
***/
pub fn is_new_modbus(line: String) -> bool {
    if line.contains("[modbus.") {
        return true;
    } else {
        return false;
    }
}

/***
* Function is_empty_modbus:
*
* Purpose:
* Checks if modbus is default
***/
pub fn is_empty_modbus(mod_config: ModbusConfig) -> bool {
    /* Creating empty struct */
    let mut empty_modbus = ModbusConfig::default();

    if  empty_modbus == mod_config {
        return true;
    } else {
        return false;
    }
}

/***
* Function parse_modbus:
*
* Purpose:
* Parses line that involve modbus
***/
pub fn parse_modbus(line: String, mod_config: ModbusConfig)
{
    let v: Vec<&str> = line.split("=").collect();

    let comparitor = v[0].trim().to_string();
    let value = v[1].trim().to_string();
    
    /* ***REFACTOR this is awful*** */
    if comparitor == "baud_rate" {
        mod_config.baud_rate = value.parse::<i32>().unwrap();
    } else
	if comparitor == "data_bits" {
        mod_config.data_bits = value.parse::<i32>().unwrap();
    } else
	if comparitor == "parity" {
        mod_config.parity = value;
    } else
	if comparitor == "stop_bits" {
        mod_config.stop_bits = value.parse::<i32>().unwrap();
    } else
	if comparitor== "slave_id" {
        mod_config.slave_id = value.parse::<i8>().unwrap();
    } else
	if comparitor == "serial_timeout" {
        mod_config.serial_timeout = value.parse::<i32>().unwrap();
    } else
	if comparitor == "address" {
        mod_config.address = value;
    } else
	if comparitor == "sampling_interval" {
        mod_config.sampling_interval = value.parse::<i32>().unwrap();
    }
}

/***
* Function parse_sensor:
*
* Purpose:
* Parses line that involve sensor
***/
pub fn parse_sensor(line: String, sensor_map: HashMap<String, bool>)
{
    let v: Vec<&str> = line.split("=").collect();
    if v[1].trim().to_string() == "false" {
        sensor_map.insert(v[0].trim().to_string(), false);
    } else {
        sensor_map.insert(v[0].trim().to_string(), true);
    }
    
}
