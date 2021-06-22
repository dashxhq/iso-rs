use crate::codegen::*;
use serde_json::{from_str, Value};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub type Timezones = HashMap<String, Vec<String>>;

pub fn get_time() -> Result<Timezones, Box<dyn Error>> {
    let mut timezones =
        File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/build/timezones.json")).unwrap();
    let mut data = String::new();
    timezones.read_to_string(&mut data).unwrap();
    let mut map: Timezones = HashMap::new();
    let parsed: Value = from_str(data.as_str())?;
    if let Some(zones) = parsed.get("zones") {
        if let Some(zone_data) = zones.as_array() {
            for zone in zone_data {
                if let Some(code) = zone.get("countryCode") {
                    if let Some(zone_name) = zone.get("zoneName") {
                        map.entry(code.to_string())
                            .or_default()
                            .push(zone_name.to_string());
                    }
                }
            }
        }
    }
    Ok(map)
}

pub fn timezone_vec(timezones: Vec<String>) -> Vec<String> {
    let mut vec = Vec::new();
    for timezone in timezones.iter() {
        vec.push(timezone_struct(timezone.to_string()));
    }
    vec
}
