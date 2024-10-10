// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;
// A structure to store the goal details of a team.
#[derive(Debug, Clone)] // 添加 Clone 特性
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores: HashMap<String, Team> = [
        ("England", Team { goals_scored: 0, goals_conceded: 0 }),
        ("France", Team { goals_scored: 0, goals_conceded: 0 }),
        ("Germany", Team { goals_scored: 0, goals_conceded: 0 }),
        ("Italy", Team { goals_scored: 0, goals_conceded: 0 }),
        ("Poland", Team { goals_scored: 0, goals_conceded: 0 }),
        ("Spain", Team { goals_scored: 0, goals_conceded: 0 }),
    ]
    .iter()
    .map(|(name, team)| (name.to_string(), team.clone())) // 移除模式匹配中的 &
    .collect();

    for r in results.lines() {
        let parts: Vec<&str> = r.split(',').collect();
        if parts.len() != 4 {
            panic!("Invalid result format: {}", r);
        }
        let team_1_name = parts[0].trim().to_string();
        let team_1_score: u8 = parts[2].trim().parse().unwrap();
        let team_2_name = parts[1].trim().to_string();
        let team_2_score: u8 = parts[3].trim().parse().unwrap();

        match scores.get_mut(&team_1_name) {
            Some(team_1) => {
                team_1.goals_scored += team_1_score;
                team_1.goals_conceded += team_2_score;
            },
            None => panic!("Unknown team: {}", team_1_name),
        }

        match scores.get_mut(&team_2_name) {
            Some(team_2) => {
                team_2.goals_scored += team_2_score;
                team_2.goals_conceded += team_1_score;
            },
            None => panic!("Unknown team: {}", team_2_name),
        }
    }

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }
    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }
    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
