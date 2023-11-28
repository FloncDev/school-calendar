use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use reqwest::header::HeaderValue;
use serde_json::{json, Value};
use std::iter::zip;

use crate::edulink::{EduLink, BASE_URL};
use crate::models::Lesson;

impl EduLink {
    fn generate_times(
        &mut self,
        lesson_length: Duration,
        mut start_time: NaiveTime,
    ) -> Vec<NaiveTime> {
        let mut lesson_times: Vec<NaiveTime> = Vec::new();

        for i in 1..=6 {
            // 6 Lessons + Lunch + After School
            match i {
                3 => {
                    start_time += Duration::minutes(15); // Break
                }
                5 => {
                    start_time += Duration::minutes(50); // Lunch
                }
                _ => {}
            }

            lesson_times.push(start_time);
            start_time += lesson_length;
        }

        lesson_times
    }

    pub async fn get_timetable(&mut self) -> Vec<Lesson> {
        let mut headers = self.get_headers();
        headers.insert(
            "X-API-Method",
            HeaderValue::from_static("EduLink.Timetable"),
        );

        let now = Utc::now().date_naive();

        let req_json = json!({
            "jsonrpc": "2.0",
            "params": {
                "date": now.format("%Y-%m-%d").to_string(),
                "learner_id": self.learner_id
            },
            "id": 1
        });

        let response = self
            .client
            .post(BASE_URL)
            .headers(headers)
            .json(&req_json)
            .send()
            .await
            .unwrap();

        let json: Value = response.json().await.unwrap();
        let result: &Value = json.get("result").unwrap();
        let weeks: &Value = result.get("weeks").unwrap();

        let monday_start_times = self.generate_times(
            Duration::minutes(45),
            NaiveTime::from_hms_opt(9, 25, 0).unwrap(),
        );

        let normal_start_times = self.generate_times(
            Duration::minutes(50),
            NaiveTime::from_hms_opt(8, 55, 0).unwrap(),
        );

        let mut lessons: Vec<Lesson> = vec![];

        for week in weeks.as_array().unwrap() {
            let days = week.get("days").unwrap().as_array().unwrap();

            for day in days {
                let date = day.get("date").unwrap().as_str().unwrap();
                let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
                let weekday = day.get("name").unwrap().as_str().unwrap();

                let start_times = if weekday == "Monday" {
                    &monday_start_times
                } else {
                    &normal_start_times
                };

                for (start_time, lesson) in
                    zip(start_times, day.get("lessons").unwrap().as_array().unwrap())
                {
                    let subject = lesson
                        .get("teaching_group")
                        .unwrap()
                        .get("subject")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let teacher = lesson.get("teachers").unwrap().as_str().unwrap();

                    let room = match lesson.get("room").unwrap().get("name").unwrap().as_str() {
                        Some(val) => val,
                        None => "GYM", // For some reason the gym has no room
                    };

                    let start_datetime = NaiveDateTime::new(date, start_time.to_owned());

                    lessons.push(Lesson {
                        start_time: start_datetime,
                        end_time: start_datetime + {
                            if weekday == "Monday" {
                                Duration::minutes(45)
                            } else {
                                Duration::minutes(50)
                            }
                        },
                        lesson: subject.to_string(),
                        teacher: teacher.to_string(),
                        room: room.to_string(),
                    })
                }
            }
        }

        lessons
    }
}
