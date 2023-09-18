use crate::models::Week;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use icalendar::{parser::read_calendar, Calendar, Component, Event, EventLike};
use serde_json::{self, Value};
use std::{fs, time::SystemTime};

pub fn json_to_cal() -> Calendar {
    let data = fs::read_to_string("data.json").unwrap();
    let json: Value = serde_json::from_str(&data).expect("Invalid JSON");

    let json_data = json.as_object().unwrap();

    let week = Week::new(&json_data.get("schedule").unwrap().clone()).unwrap();
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
                        .done(),
                );
            }
        }
    }

    let mut period_7s = after_school(&json_data["after_school"]).unwrap();
    cal.append(&mut period_7s);

    cal
}

#[doc = "Create calendar from json or get previous one from cache if less than an hour old."]
pub fn get_calander() -> Calendar {
    let metadata = match fs::metadata("cal.ics") {
        Ok(meta) => meta,
        Err(_) => {
            let cal = json_to_cal();
            fs::write("cal.ics", format!("{}", cal)).unwrap();
            return cal;
        }
    };

    let now = SystemTime::now();

    if now
        .duration_since(metadata.modified().unwrap())
        .unwrap()
        .as_secs()
        > 60 * 60
    {
        let cal = json_to_cal();
        fs::write("cal.ics", format!("{}", cal)).unwrap();
        return cal;
    }

    read_calendar(fs::read_to_string("cal.ics").unwrap().as_str())
        .unwrap()
        .into()
}

pub fn after_school(json: &Value) -> Option<Calendar> {
    let mut cal = Calendar::new();

    let weeks = json.as_array()?;

    let now = Local::now().date_naive();
    let isoweek = now.iso_week().week();

    for week_index in 0..2 {
        let week = weeks[week_index].as_array()?;

        for (day_index, day) in week.iter().enumerate() {
            let day = day.as_array()?;

            for class in day {
                let class = class.as_object()?;

                let start_time =
                    NaiveTime::parse_from_str(class["start_time"].as_str()?, "%I:%M %p").unwrap();

                let weekday = Weekday::try_from(day_index as u8).unwrap();
                let day = NaiveDate::from_isoywd_opt(now.year(), isoweek, weekday)?;

                let week_offset = (isoweek + (week_index as u32) + 1) % 2;

                let start_dt =
                    NaiveDateTime::new(day, start_time) + Duration::weeks(week_offset as i64);

                cal.push(
                    Event::new()
                        .summary(class["name"].as_str()?)
                        .starts(start_dt)
                        .ends(start_dt + Duration::hours(1))
                        .done(),
                );
            }
        }
    }

    Some(cal)
}
