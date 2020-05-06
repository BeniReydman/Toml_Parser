/* Never ended up using this, might be useful in the future though... */

// This enum represents possible values stored in a toml
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}