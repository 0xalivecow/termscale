use serde_json::{Result, Value};

//mod structs;

pub fn deserialise_json(raw_data: &str) -> Result<Value> {
    let v: Value = serde_json::from_str(raw_data)?;
    Ok(v)
}

pub fn ext_self_info(val: &Value) {}
