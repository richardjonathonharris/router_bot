#[cfg(test)]
mod tests {
    use super::super::{*};

    #[test]
    fn can_determine_when_action_is_not_labeled() {
        let pull_request_action = github::PullRequestEvent {
            action: "".to_string(),
            pull_request: github::default_pull_request(),
            label: github::default_label(),
        };

        assert_eq!(pull_request_action.valid_label_application(), false);
    }
    
    #[test]
    fn can_determine_when_action_is_labeled() {
        let pull_request_action = github::PullRequestEvent {
            action: "labeled".to_string(),
            pull_request: github::default_pull_request(),
            label: github::default_label(),
        };

        assert_eq!(pull_request_action.valid_label_application(), true);
    }

    #[test]
    fn formats_message_correctly() {
        let pull_request_action = github::PullRequestEvent {
            action: "labeled".to_string(),
            pull_request: github::PullRequest {
                number: 12345,
                title: "Test PR".to_string(),
                url: "http://test.pr/".to_string(),
                user: github::GithubUser {
                    login: "user".to_string(),
                },
            },
            label: github::default_label(),
        };

        assert_eq!(pull_request_action.generate_message(), "Pull Request #12345 at http://test.pr/ is ready for review!");
    }
}
