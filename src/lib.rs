use std::fs;

use chrono::{NaiveDate, NaiveTime, Local, Weekday, Datelike, NaiveDateTime, Duration};
use icalendar::{Calendar, Event, Component, EventLike};
use rocket::fs::NamedFile;

#[macro_use]
extern crate rocket;

pub mod parser;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            get_calander
        ]
    )
}

#[get("/cal.ics")]
pub async fn get_calander() -> Option<NamedFile> {
    let json = parser::parse();
    let week = json.as_array()?;

    let mut cal = Calendar::new();

    cal.push(
        Event::new()
        .summary("This is an event :)")
        .all_day(NaiveDate::from_ymd_opt(2023, 10, 4)?)
        .done()
    );

    for (index, day) in week.iter().enumerate() {
        let classes = day.as_array()?;
        for class in classes {
            let obj = class.as_object()?;

            let start_time = NaiveTime::parse_from_str(obj["start_time"].as_str()?, "%I:%M %p").unwrap();
            let end_time = NaiveTime::parse_from_str(obj["end_time"].as_str()?, "%I:%M %p").unwrap();

            let now = Local::now();
            let now_date = now.date_naive();

            let weekday = Weekday::try_from(index as u8).unwrap();
            let day = NaiveDate::from_isoywd_opt(now_date.year(), now_date.iso_week().week(), weekday)?;

            let mut start_dt = NaiveDateTime::new(day, start_time);
            let mut end_dt = NaiveDateTime::new(day, end_time);

            cal.push(
                Event::new()
                .summary(&obj["name"].as_str()?)
                .location(&obj["classroom"].as_str()?)
                .starts(start_dt)
                .ends(end_dt)
                .done()
            );

            // Also add for next week
            start_dt += Duration::weeks(1);
            end_dt += Duration::weeks(1);

            cal.push(
                Event::new()
                .summary(&obj["name"].as_str()?)
                .location(obj["classroom"].as_str()?)
                .starts(start_dt)
                .ends(end_dt)
                .done()
            );
        }
    }

    fs::write("cal.ical", format!("{}", cal).as_bytes()).expect("Couldn't write to cal.ical");

    match NamedFile::open("cal.ical").await {
        Ok(val) => Some(val),
        Err(_) => None
    }
}