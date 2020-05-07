use std::collections::HashMap;
 
// ModbusConfig is the config for sensor using modbus
#[derive(PartialEq, PartialOrd, Clone, Debug)]  // This allows comparisons between structs
pub struct ModbusConfig {
    pub baud_rate:         i32,
	pub data_bits:         i32,
	pub parity:            String,
	pub stop_bits:         i32,
	pub slave_id:          i8, // byte
	pub serial_timeout:    i32,
	pub address:           String,
	pub sampling_interval: i32
}

// Create a default empty struct
impl Default for ModbusConfig {
    fn default () -> ModbusConfig {
		ModbusConfig{baud_rate: 0, data_bits: 0, parity:"".to_string(), stop_bits:0, 
				slave_id:0, serial_timeout:0,  address:"".to_string(), sampling_interval:0}
    }
}

// Config is the config for initialize the server
// Contain sensor initialize information
pub struct Config {
	pub sensor: HashMap<String, bool>,
	pub modbus: HashMap<String, ModbusConfig>
}