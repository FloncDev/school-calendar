use chrono::NaiveDate;

pub struct Homework {
    pub activity: String,
    pub subject: String,
    pub due: NaiveDate,
    pub set_by: String,
}
