use chrono::{NaiveDateTime, NaiveTime, Duration, Local, Weekday, NaiveDate, Datelike};
use serde_json::Value;

pub struct Class {
    pub name: String,
    pub start: NaiveDateTime,
    pub length: Duration,
    pub classroom: String
}

pub struct Day {
    pub classes: Vec<Class>
}

pub struct Week {
    pub days: Vec<Day>
}

impl Week {
    pub fn new(json: Value) -> Option<Week> {
        let week = json.as_array()?;
        let mut days: Vec<Day> = vec![];
        
        for (day_index, day) in week.iter().enumerate() {
            let classes = day.as_array()?;
            let mut day_classes: Vec<Class> = vec![];
            let mut skip = false;

            for (class_index, class) in classes.iter().enumerate() {
                if skip {
                    skip = false;
                    continue;
                }
                let obj = class.as_object()?;

                let start_time = NaiveTime::parse_from_str(obj["start_time"].as_str()?, "%I:%M %p").unwrap();
                let mut end_time = NaiveTime::parse_from_str(obj["end_time"].as_str()?, "%I:%M %p").unwrap();

                
                match classes.get(class_index + 1) {
                    Some(next_class) => {
                        let next_obj = next_class.as_object()?;
                        
                        if next_obj["name"].as_str()? == obj["name"].as_str()? {
                            end_time = NaiveTime::parse_from_str(next_obj["end_time"].as_str()?, "%I:%M %p").unwrap();
                            skip = true;
                        }
                    },
                    None => {}
                }
                
                let length = end_time - start_time;

                let date = Local::now().date_naive();
                let weekday = Weekday::try_from(day_index as u8).unwrap();
                let day = NaiveDate::from_isoywd_opt(date.year(), date.iso_week().week(), weekday)?;

                let start_dt = NaiveDateTime::new(day, start_time);

                day_classes.push(
                    Class {
                        name: obj["name"].as_str()?.to_string(),
                        start: start_dt,
                        length,
                        classroom: obj["classroom"].as_str()?.to_string()
                    }
                )
            }

            // days.push();

            days.push(
                Day {
                    classes: day_classes
                }
            )
        }

        Some(Week { days })
    }
}

pub struct Homework {
    pub summary: String,
    pub class: String,
    pub due: NaiveDate,
    pub estimated_length: Duration
}

impl Homework {
    #[doc = "Example shorthand: CS 20/09 15 The rest of this is a summary"]
    pub fn from_shorthand(shorthand: String) -> Homework {
        let mut split = shorthand.split(" ").into_iter();

        // TODO: Get class from another function
        let class = String::from("Computer Science 11B/Cs1");
        split.next();
        
        let due = NaiveDate::parse_from_str(split.next().unwrap(), "%d/%m").expect("Date is in wrong format");

        let length = Duration::minutes(split.next().unwrap().parse::<i64>().unwrap());

        let summary: String = split.map(|x| x.to_owned() + " ").collect();

        Homework {
            summary: summary.trim().to_owned(),
            class,
            due,
            estimated_length: length
        }
    }
}