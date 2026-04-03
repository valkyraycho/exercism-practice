use std::{cmp::Ordering, collections::HashMap};

#[derive(Default)]
struct TeamStat {
    wins: u32,
    losses: u32,
    draws: u32,
}

impl TeamStat {
    fn mp(&self) -> u32 {
        self.wins + self.losses + self.draws
    }

    fn points(&self) -> u32 {
        self.wins * 3 + self.draws
    }
}

pub fn tally(match_results: &str) -> String {
    let mut team_stats_map: HashMap<&str, TeamStat> = HashMap::new();
    let mut result = vec![String::from(
        "Team                           | MP |  W |  D |  L |  P",
    )];

    for line in match_results.lines() {
        let components: Vec<&str> = line.split(';').collect();
        let (team1, team2, game_result) = (components[0], components[1], components[2]);
        match game_result {
            "win" => {
                team_stats_map.entry(team1).or_default().wins += 1;
                team_stats_map.entry(team2).or_default().losses += 1
            }
            "loss" => {
                team_stats_map.entry(team1).or_default().losses += 1;
                team_stats_map.entry(team2).or_default().wins += 1
            }
            "draw" => {
                team_stats_map.entry(team1).or_default().draws += 1;
                team_stats_map.entry(team2).or_default().draws += 1
            }
            _ => unreachable!(),
        }
    }
    let mut standings: Vec<_> = team_stats_map.iter().collect();
    standings.sort_by(|(team1, stat1), (team2, stat2)| {
        match stat1.points().cmp(&stat2.points()).reverse() {
            Ordering::Equal => team1.cmp(team2),
            ordering => ordering,
        }
    });
    for (team, stats) in standings {
        result.push(format!(
            "{:<31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team,
            stats.mp(),
            stats.wins,
            stats.draws,
            stats.losses,
            stats.points()
        ));
    }

    result.join("\n")
}
