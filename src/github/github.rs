use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GithubUser {
    login: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Label {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PullRequest {
    number: i32,
    title: String,
    url: String,
    user: GithubUser,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestEvent {
    action: String,
    pull_request: PullRequest,
    label: Label,
}

impl PullRequestEvent {
    pub fn generate_message(&self) -> String {
        format!("Pull Request #{} at {} is ready for review!", &self.pull_request.number, &self.pull_request.url)
    }
}

