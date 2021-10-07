use std::env;

mod slack;
mod slack_test;

pub fn post_message() {
    let key = "SLACK_CHANNEL";

    let channel = match env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("Could not find envvar for SLACK_CHANNEL. Error: {}", e),
    };
    let team = "team";
    let message = "Hi team, I sent this from Rust! I am really excited to be here!";

    let config = slack::Config::new(&channel, team);
    let payload = slack::Payload::new(&config, message);
    let _result = payload.post();
}
