use chrono::{Duration, Local, NaiveDateTime};
use dotenvy::dotenv;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT},
    Client,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::env;

const BASE_URL: &str = "https://www9.edulinkone.com/api/";

#[derive(Debug, Deserialize)]
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

        println!("{}", login_json);

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

        EduLink {
            auth_token: result.authtoken,
            expires: (Local::now() + Duration::seconds(1800)).naive_local(),
            client,
            learner_id,
        }
    }

    pub async fn from_json(json: TokenJSON) -> Self {
        let expires = NaiveDateTime::from_timestamp_opt(json.expires, 0).unwrap();

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
}
