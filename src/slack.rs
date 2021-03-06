use log::{info};
use serde::Serialize;
use std::env;

const ENDPOINT: &str  = "https://slack.com/api/chat.postMessage";

pub struct Config {
    channel: String,
}

impl Config {
    pub fn new(channel: String) -> Config {
        Config {
            channel,
        }
    }
}

#[derive(Serialize)]
pub struct Payload {
    channel: String,
    text: String,
    blocks: Vec<Block>,
}

#[derive(Serialize)]
pub struct TextType {
    #[serde(rename(serialize = "type"))]
    text_type: String,
    text: String,
}

#[derive(Serialize)]
pub struct Block {
    #[serde(rename(serialize = "type"))]
    block_type: String,
    text: TextType
}

impl Payload {
    pub fn new(config: Config, text: String, markdown_text: String) -> Payload {
        let text_block = TextType{ text_type: "mrkdwn".to_string(), text: markdown_text };
        let block = Block { block_type: "section".to_string(), text: text_block };
        Payload {
            channel: config.channel,
            text,
            blocks: vec![block],
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub async fn post(&self) -> Result<(), reqwest::Error> {
        let key = "SLACK_BOT_TOKEN";

        let mut bearer_token: String = "Bearer ".to_owned();
        let token = env::var(key).expect("Could not find envvar for SLACK_BOT_TOKEN");
        bearer_token.push_str(&token);
        let request = String::from(&self.to_json());
        let client = reqwest::ClientBuilder::new().build()?;

        info!("Sending message body to {}: {}", ENDPOINT, request);

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

#[cfg(test)]
mod tests {
    use super::*;

    const CHANNEL: &str = "test-channel";
    const MESSAGE: &str = "test-message";

    #[test]
    fn can_create_slack_config() {
        let config = Config::new(String::from(CHANNEL));
        assert_eq!(config.channel, CHANNEL);
    }

    #[test]
    fn can_create_slack_payload() {
        let config = Config::new(String::from(CHANNEL));
        let payload = Payload::new(config, String::from(MESSAGE), String::from(MESSAGE));
        assert_eq!(payload.channel, CHANNEL);
        assert_eq!(payload.text, MESSAGE);
        assert_eq!(payload.blocks[0].block_type, "section");
        assert_eq!(payload.blocks[0].text.text_type, "mrkdwn");
        assert_eq!(payload.blocks[0].text.text, MESSAGE);
    }

    #[test]
    fn can_serialize_slack_payload_to_json() {
        let config = Config::new(String::from(CHANNEL));
        let payload = Payload::new(config, String::from(MESSAGE), String::from(MESSAGE));
        assert_eq!(String::from("{\"channel\":\"test-channel\",\"text\":\"test-message\",\"blocks\":[{\"type\":\"section\",\"text\":{\"type\":\"mrkdwn\",\"text\":\"test-message\"}}]}"), payload.to_json())
    }
}
