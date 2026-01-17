// A list of scores (one per line) of a soccer match is given. Each line is of
// the form "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: "England,France,4,2" (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, the total
// number of goals the team scored, and the total number of goals the team
// conceded.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

struct MatchResult<'a> {
    team_a_name: &'a str,
    team_b_name: &'a str,
    team_a_score: u8,
    team_b_score: u8,
}

impl<'a> MatchResult<'a> {
    fn new(team_a_name: &'a str, team_b_name: &'a str, team_a_score: u8, team_b_score: u8) -> Self {
        Self {
            team_a_name,
            team_b_name,
            team_a_score,
            team_b_score,
        }
    }

    fn insert_into_scores(&self, scores: &mut HashMap<&'a str, TeamScores>) {
        scores
            .entry(self.team_a_name)
            .and_modify(|e| {
                e.goals_scored += self.team_a_score;
                e.goals_conceded += self.team_b_score;
            })
            .or_insert(TeamScores {
                goals_scored: self.team_a_score,
                goals_conceded: self.team_b_score,
            });

        scores
            .entry(self.team_b_name)
            .and_modify(|e| {
                e.goals_scored += self.team_b_score;
                e.goals_conceded += self.team_a_score;
            })
            .or_insert(TeamScores {
                goals_scored: self.team_b_score,
                goals_conceded: self.team_a_score,
            });
    }
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // TODO: Populate the scores table with the extracted details.
        // Keep in mind that goals scored by team 1 will be the number of goals
        // conceded by team 2. Similarly, goals scored by team 2 will be the
        // number of goals conceded by team 1.

        // NOTE: We use `unwrap` because we didn't deal with error handling yet.
        let match_result = MatchResult {
            // Bc we are using an iterator, the order of these fields matters!!!
            team_a_name: split_iterator.next().unwrap(),
            team_b_name: split_iterator.next().unwrap(),
            team_a_score: split_iterator.next().unwrap().parse().unwrap(),
            team_b_score: split_iterator.next().unwrap().parse().unwrap(),
        };

        match_result.insert_into_scores(&mut scores);
    }

    scores
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(
            ["England", "France", "Germany", "Italy", "Poland", "Spain"]
                .into_iter()
                .all(|team_name| scores.contains_key(team_name))
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
