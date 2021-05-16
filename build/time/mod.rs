use crate::codegen::*;
use reqwest::blocking::get;
use serde_json::{from_str, Value};
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

const API_KEY: &'static str = "0JJCCBNXW8K7";

pub type Timezones = HashMap<String, Vec<Zone>>;

#[derive(Debug, Default, Clone)]
pub struct Zone {
    zone_name: String,
    offset: String,
}

pub fn get_time() -> Result<Timezones, Box<dyn Error>> {
    let url = format!(
        "http://api.timezonedb.com/v2.1/list-time-zone?key={}&format=json",
        API_KEY
    );
    let data = get(url)?.text()?;
    let mut map: Timezones = HashMap::new();
    let parsed: Value = from_str(data.as_str())?;
    if let Some(zones) = parsed.get("zones") {
        if let Some(zone_data) = zones.as_array() {
            for zone in zone_data {
                if let Some(code) = zone.get("countryCode") {
                    if let Some(zone_name) = zone.get("zoneName") {
                        map.entry(code.to_string()).or_default().push(Zone {
                            zone_name: zone_name.to_string(),
                            offset: zone
                                .get("gmtOffset")
                                .unwrap_or(&Value::from_str("0")?)
                                .to_string(),
                        });
                    }
                }
            }
        }
    }
    Ok(map)
}

fn second_offset_to_utc_offset(offset: String) -> String {
    let mut sign = &offset[0..1];
    if sign != "-" {
        sign = "+";
    }
    let integer = &offset[1..].parse::<f64>();
    match integer {
        Ok(x) => {
            let hours = x / 3600.0;
            let mut minutes = (hours.fract() * 60.0).floor().to_string();
            if minutes == "0" {
                minutes.push('0')
            }
            format!("UTC{}{}:{}", sign, hours.floor(), minutes)
        }
        Err(_) => String::new(),
    }
}

pub fn timezone_vec(timezones: Vec<Zone>) -> Vec<String> {
    let mut vec = Vec::new();
    for timezone in timezones.iter() {
        vec.push(timezone_struct(
            second_offset_to_utc_offset(timezone.offset.clone()),
            timezone.zone_name.clone(),
        ))
    }
    vec
}
