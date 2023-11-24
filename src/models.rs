use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
pub struct Homework {
    pub activity: String,
    pub subject: String,
    pub due: NaiveDate,
    pub set_by: String,
}

#[derive(Debug)]
pub struct Lesson {
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub lesson: String,
    pub teacher: String,
    pub room: String,
}
