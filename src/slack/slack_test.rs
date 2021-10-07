#[cfg(test)]
mod tests {
    use super::super::{*};

    const CHANNEL: &str = "test-channel";
    const TEAM: &str = "test-team";
    const MESSAGE: &str = "test-message";

    #[test]
    fn can_create_slack_config() {
        let config = slack::Config::new(CHANNEL, TEAM);
        assert_eq!(config.channel, CHANNEL);
        assert_eq!(config.team, TEAM);
    }

    #[test]
    fn can_create_slack_payload() {
        let config = slack::Config::new(CHANNEL, TEAM);
        let payload = slack::Payload::new(&config, MESSAGE);
        assert_eq!(payload.channel, CHANNEL);
        assert_eq!(payload.text, MESSAGE);
    }

    #[test]
    fn can_serialize_slack_payload_to_json() {
        let config = slack::Config::new(CHANNEL, TEAM);
        let payload = slack::Payload::new(&config, MESSAGE);
        assert_eq!(String::from("{\"channel\":\"test-channel\",\"text\":\"test-message\"}"), payload.to_json())
    }
}
