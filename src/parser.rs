use std::fs;
use serde_json;

pub fn parse() -> serde_json::Value {
    let content = match fs::read_to_string("schedule.json") {
        Ok(val) => val,
        Err(err) => {panic!("{}", err)}
    };

    serde_json::from_str(&content).expect("Invalid Json")
}