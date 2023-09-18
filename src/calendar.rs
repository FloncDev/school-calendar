use std::{fs, time::SystemTime};
use chrono::Duration;
use icalendar::{Calendar, Event, Component, EventLike, parser::read_calendar};
use serde_json::{self, Value};
use crate::models::Week;

pub fn json_to_cal() -> Calendar {
    let data = fs::read_to_string("data.json").unwrap();
    let json: Value = serde_json::from_str(&data).expect("Invalid JSON");

    let json_data = json.as_object().unwrap();
    
    let week = Week::new(json_data.get("schedule.json").unwrap().clone()).unwrap();
    let mut cal: Calendar = Calendar::new();

    for day in week.days {
        for class in day.classes {
            for i in 0..=1 {
                let start = class.start + Duration::weeks(i);

                cal.push(
                    Event::new()
                    .summary(class.name.as_str())
                    .starts(start)
                    .ends(start + class.length)
                    .location(class.classroom.as_str())
                    .done()
                );
            }
        }
    }

    cal
}

#[doc = "Create calendar from json or get previous one from cache if less than an hour old."]
pub fn get_calander() -> Calendar {
    let metadata = match fs::metadata("cal.ics") {
        Ok(meta) => {meta},
        Err(_) => {
            let cal = json_to_cal();
            fs::write("cal.ics", format!("{}", cal)).unwrap();
            return cal
        }
    };

    let now = SystemTime::now();

    if now.duration_since(metadata.modified().unwrap()).unwrap().as_secs() > 60 * 60 {
        let cal = json_to_cal();
        fs::write("cal.ics", format!("{}", cal)).unwrap();
        return cal
    }

    read_calendar(fs::read_to_string("cal.ics").unwrap().as_str()).unwrap().into()
}