use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubUser {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequest {
    pub number: i32,
    pub title: String,
    pub url: String,
    pub user: GithubUser,
}

pub fn default_pull_request() -> PullRequest {
    PullRequest {
        number: 0,
        title: "".to_string(),
        url: "".to_string(),
        user: GithubUser {
            login: "".to_string(),
        }
    }
}

pub fn default_label() -> Label {
    Label {
        name: "".to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestEvent {
    #[serde(default)]
    pub action: String,
    #[serde(default = "default_pull_request")]
    pub pull_request: PullRequest,
    #[serde(default = "default_label")]
    pub label: Label,
}

impl PullRequestEvent {
    pub fn generate_message(&self) -> String {
        format!("Pull Request #{} at {} is ready for review!", &self.pull_request.number, &self.pull_request.url)
    }

    pub fn valid_label_application(&self) -> bool {
        &self.action == "labeled"
    }
}

