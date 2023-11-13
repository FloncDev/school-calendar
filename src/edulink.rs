use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use dotenvy::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{env, fs::File};

use crate::models::Homework;

const BASE_URL: &str = "https://www9.edulinkone.com/api/";

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenJSON {
    token: String,
    expires: String,
    learner_id: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Learner {
    id: String,
}

#[derive(Debug, Deserialize)]
struct LoginResult {
    authtoken: String,
    children: Vec<Learner>,
}

#[derive(Debug, Deserialize)]
struct APIHomework {
    activity: String,
    subject: String,
    due_date: String,
    set_by: String,
}

#[derive(Debug)]
pub struct EduLink {
    pub auth_token: String,
    pub expires: NaiveDateTime,
    pub client: Client,
    pub learner_id: String,
}

impl EduLink {
    pub async fn new() -> Self {
        match dotenv() {
            Ok(_) => {}
            Err(_) => {}
        };
        let client = Client::new();

        let username: String =
            env::var("EDULINK_USERNAME").expect("Could not find EDULINK_USERNAME env variable.");
        let password: String =
            env::var("EDULINK_PASSWORD").expect("Could not find EDULINK_PASSWORD env variable.");
        let establishment_id: String =
            env::var("EDULINK_ESTABLISHMENT_ID").expect("Could not find EDULINK_ESTABLISHMENT_ID.");

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"));
        headers.insert("X-API-Method", HeaderValue::from_static("EduLink.Login"));

        let login_json = json!({
            "jsonrpc": "2.0",
            "params": {
                "username": username,
                "password": password,
                "establishment_id": establishment_id
            },
            "id": "1"
        });

        println!("Sending new login request");
        let response = client
            .post(BASE_URL)
            .headers(headers)
            .json(&login_json)
            .send()
            .await
            .unwrap();

        let data: Value = response.json().await.unwrap();

        let result: LoginResult =
            serde_json::from_value(data.get("result").unwrap().clone()).unwrap();

        let learner_id = result.children[0].clone().id;
        let expires = (Local::now() + Duration::seconds(1800)).naive_local();

        // Save to file
        serde_json::to_writer(
            &File::create("./.token.json").unwrap(),
            &TokenJSON {
                token: result.authtoken.clone(),
                expires: expires.timestamp().to_string(),
                learner_id: learner_id.clone(),
            },
        )
        .expect("Could not write to file");

        EduLink {
            auth_token: result.authtoken,
            expires,
            client,
            learner_id,
        }
    }

    pub async fn from_json(json: TokenJSON) -> Self {
        let expires = NaiveDateTime::from_timestamp_opt(json.expires.parse().unwrap(), 0).unwrap();

        if expires < Local::now().naive_local() {
            return EduLink::new().await;
        }

        let client = Client::new();

        EduLink {
            auth_token: json.token,
            expires,
            client,
            learner_id: json.learner_id,
        }
    }

    pub async fn get_homework(&mut self) -> Vec<Homework> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36"));
        headers.insert("X-API-Method", HeaderValue::from_static("EduLink.Homework"));
        let formatted_token = format!("Bearer {}", self.auth_token);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::try_from(formatted_token).unwrap(),
        );

        let homework_json = json!({
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
            .json(&homework_json)
            .send()
            .await
            .unwrap();

        let data: Value = response.json().await.unwrap();

        // let result: LoginResult =
        //     serde_json::from_value(data.get("result").unwrap().clone()).unwrap();

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
