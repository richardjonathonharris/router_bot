#[macro_use] extern crate rocket;

use log::{error, info};
use rocket::{Build, Rocket};
use rocket::http::Status;
use rocket::serde::json::{Json};
use simplelog::*;
use std::env;

mod github;
mod slack;

#[post("/github_webhooks", format="application/json", data = "<input>")]
async fn receive_webhook(input: Json<github::github::PullRequestEvent>) -> Status {
    info!("Received input from the outside world! Info is {:?}", input);
    let key = "SLACK_CHANNEL";
    let team = "team";
    let channel = env::var(key).expect("Could not find envvar for SLACK_CHANNEL");
    let message = input.generate_message();

    let config = slack::slack::Config::new(&channel, team);
    let payload = slack::slack::Payload::new(&config, &message);
    let _result = payload.post().await;
    Status::Accepted
}

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![receive_webhook])
}

#[rocket::main]
async fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();
    info!("Beginning web server, have fun!");
    if let Err(e) = rocket().launch().await {
        error!("Web server unable to start! :(");
        drop(e);
    };
}
