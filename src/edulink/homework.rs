use chrono::NaiveDate;
use reqwest::header::HeaderValue;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::edulink::{EduLink, BASE_URL};
use crate::models::Homework;

#[derive(Debug, Deserialize)]
struct APIHomework {
    activity: String,
    subject: String,
    due_date: String,
    set_by: String,
}

impl EduLink {
    pub async fn get_homework(&mut self) -> Vec<Homework> {
        let mut headers = self.get_headers();
        headers.insert("X-API-Method", HeaderValue::from_static("EduLink.Homework"));

        let req_json = json!({
            "jsonrpc": "2.0",
            "params": {
                "format": 2,
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

        let data: Value = response.json().await.unwrap();

        let homeworks: Vec<APIHomework> =
            serde_json::from_value(data["result"]["homework"]["current"].clone()).unwrap();

        homeworks
            .iter()
            .map(|homework| Homework {
                activity: homework.activity.clone(),
                subject: homework.subject.clone(),
                due: NaiveDate::parse_from_str(homework.due_date.as_str(), "%Y-%m-%d").unwrap(),
                set_by: homework.set_by.clone(),
            })
            .collect()
    }
}
