pub struct Team {
  label_id: i64,
  pub name: String,
  pub channel_id: String,
}

impl Team {
  pub fn new(label_id: i64, name: String, channel_id: String) -> Team {
    Team {
      label_id,
      name,
      channel_id,
    }
  }
}

pub fn filter_teams(teams: Vec<Team>, label_id: i64) -> Vec<Team> {
  teams
    .into_iter()
    .filter(|team| team.label_id == label_id)
    .collect::<Vec<Team>>()
}

#[cfg(test)]
mod tests {
  use super::*;

  const LABEL_ID: &i64 = &123456;
  const NAME: &str = "team-name";
  const CHANNEL_ID: &str = "CH123456";

  #[test]
  fn can_create_team() {
    let team = Team::new(*LABEL_ID, String::from(NAME), String::from(CHANNEL_ID));
    assert_eq!(team.label_id, *LABEL_ID);
    assert_eq!(team.name, NAME);
  }

  #[test]
  fn can_filter_vector_of_teams() {
    let teams = vec![
      Team::new(123456, "Test Team".to_string(), String::from(CHANNEL_ID)),
      Team::new(654321, "Test Team 2".to_string(), String::from(CHANNEL_ID)),
    ];

    let filtered_team = filter_teams(teams, 654321);

    assert_eq!(filtered_team.len(), 1);
    assert_eq!(filtered_team[0].label_id, 654321);
  }
}
