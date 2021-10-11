use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GithubUser {
    login: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PullRequest {
    number: i32,
    title: String,
    url: String,
    user: GithubUser,
}

fn default_pull_request() -> PullRequest {
    PullRequest {
        number: 0,
        title: "".to_string(),
        url: "".to_string(),
        user: GithubUser {
            login: "".to_string(),
        }
    }
}

fn default_label() -> Label {
    Label {
        id: 0,
        name: "".to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PullRequestEvent {
    #[serde(default)]
    action: String,
    #[serde(default = "default_pull_request")]
    pull_request: PullRequest,
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

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn can_determine_when_action_is_not_labeled() {
        let pull_request_action = PullRequestEvent {
            action: "".to_string(),
            pull_request: default_pull_request(),
            label: default_label(),
        };

        assert_eq!(pull_request_action.valid_label_application(), false);
    }

    #[test]
    fn can_determine_when_action_is_labeled() {
        let pull_request_action = PullRequestEvent {
            action: "labeled".to_string(),
            pull_request: default_pull_request(),
            label: default_label(),
        };

        assert_eq!(pull_request_action.valid_label_application(), true);
    }

    #[test]
    fn formats_message_correctly() {
        let pull_request_action = PullRequestEvent {
            action: "labeled".to_string(),
            pull_request: PullRequest {
                number: 12345,
                title: "Test PR".to_string(),
                url: "http://test.pr/".to_string(),
                user: GithubUser {
                    login: "user".to_string(),
                },
            },
            label: default_label(),
        };

        assert_eq!(pull_request_action.generate_message(), "Pull Request #12345 at http://test.pr/ is ready for review!");
    }
}
