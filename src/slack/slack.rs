use log::{info};
use serde::Serialize;
use std::env;

const ENDPOINT: &str  = "https://slack.com/api/chat.postMessage";

pub struct Config<'a> {
    pub channel: &'a str,
    pub team: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(channel: &'a str, team: &'a str) -> Config<'a> {
        Config {
            channel: channel,
            team: team,
        }
    }
}

#[derive(Serialize)]
pub struct Payload<'a> {
    pub channel: &'a str,
    pub text: &'a str,
}

impl<'a> Payload<'a> {
    pub fn new(config: &'a Config, text: &'a str) -> Payload<'a> {
        Payload {
            channel: config.channel,
            text: text,
        }
    }

    pub fn to_json(&self) -> String {
        return serde_json::to_string(&self).unwrap();
    }

    pub async fn post(&self) -> Result<(), reqwest::Error> {
        let key = "SLACK_BOT_TOKEN";

        let mut bearer_token: String = "Bearer ".to_owned();
        let token = env::var(key).expect("Could not find envvar for SLACK_BOT_TOKEN");
        bearer_token.push_str(&token);
        let request = String::from(&self.to_json());
        let client = reqwest::ClientBuilder::new().build()?;

        info!(target: "slack", "Sending body to {}: {}", ENDPOINT, request);

        let result = client
            .post(ENDPOINT)
            .header(reqwest::header::AUTHORIZATION, bearer_token)
            .header(reqwest::header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(request)
            .send()
            .await?;

        let body = result.text().await?;
        info!("Received response from {}: {}", ENDPOINT, body);

        Ok(())
    }
}
