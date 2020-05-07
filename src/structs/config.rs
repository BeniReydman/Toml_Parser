use std::collections::HashMap;
use serde::{Serialize, Deserialize};
 
// ModbusConfig is the config for sensor using modbus
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ModbusConfig {
    pub BaudRate:          Option<i32>,
	pub DataBits:          Option<i32>,
	pub Parity:            Option<String>,
	pub StopBits:          Option<i32>,
	pub SlaveID:           Option<i8>, // byte
	pub SerialTimeout:     Option<i32>,
	pub Address:           Option<String>,
	pub Len:			   Option<i32>,
	pub SamplingInterval:  Option<i32>
}

// Define construction of new ModbusConfig
#[allow(unused, non_snake_case)]
impl ModbusConfig {
	fn new(BaudRate: i32, DataBits: i32, Parity: String, StopBits: i32, SlaveID: i8, 
		SerialTimeout: i32, Address: String, Len: i32, SamplingInterval: i32) -> ModbusConfig {
			ModbusConfig {
				BaudRate: Some(BaudRate), 
				DataBits: Some(DataBits), 
				Parity: Some(Parity), 
				StopBits: Some(StopBits), 
				SlaveID: Some(SlaveID), 
				SerialTimeout: Some(SerialTimeout),  
				Address: Some(Address), 
				Len: Some(Len),
				SamplingInterval: Some(SamplingInterval)
			}
	}
}

// Create a default empty struct
impl Default for ModbusConfig {
    fn default () -> ModbusConfig {
		ModbusConfig {
			BaudRate: Some(0), 
			DataBits: Some(0), 
			Parity: Some("".to_string()), 
			StopBits: Some(0), 
			SlaveID: Some(0), 
			SerialTimeout: Some(0),  
			Address: Some("".to_string()),
			Len: Some(0), 
			SamplingInterval: Some(0)
		}
    }
}

// Config is the config for initialize the server
// Contain sensor initialize information
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
	pub sensor: HashMap<String, bool>,
	pub modbus: HashMap<String, ModbusConfig>
}