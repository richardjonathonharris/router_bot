#[macro_use] extern crate rocket;

use log::{error, info, warn};
use rocket::{Build, Rocket};
use rocket::http::Status;
use rocket::serde::json::{Json};
use simplelog::*;
use std::env;

mod github;
mod slack;
mod team;

#[post("/github_webhooks", format="application/json", data = "<input>")]
async fn receive_webhook(input: Json<github::PullRequestEvent>) -> Status {
    let teams = vec![
        team::Team::new(3428042833, "Newcomer Team".to_string(), env::var("NEWCOMER_TEAM_CHANNEL").expect("Could not find envvar for NEWCOMER_TEAM_CHANNEL")),
        team::Team::new(3428042827, "Bug Team".to_string(), env::var("BUG_TEAM_CHANNEL").expect("Could not find envvar for BUG_TEAM_CHANNEL")),
    ];

    info!("Github webhook received: {:?}", input);
    if input.valid_label_application() {
        let matching_teams = team::filter_teams(teams, input.label.id);
        if matching_teams.is_empty() {
            warn!("No team follows assigned label; skipping!: label id {}, label name {}", input.label.id, input.label.name);
        } else {
            for team in matching_teams {
                let message = input.generate_message(team.name);

                let config = slack::Config::new(team.channel_id);
                let payload = slack::Payload::new(config, message);
                let _result = payload.post().await;
            }
        }
    } else {
        warn!("Github webhook was not a label application; skipping!");
    }
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
