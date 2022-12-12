use std::collections::HashMap;

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
A Y
B X
C Z",
        )
    });

    let parsed = result.lines();
    let mut total_score_part1 = 0;
    let mut total_score_part2 = 0;
    let solver = RpcSolver::build();
    for round in parsed {
        let hands: Vec<&str> = round.split(" ").collect();
        total_score_part1 += solver.get_score_for_part1(hands[0], hands[1]);
        total_score_part2 += solver.get_score_for_part2(hands[0], hands[1]);
    }

    println!("Your score for part 1: {total_score_part1}");
    println!("Your score for part 2: {total_score_part2}");
}

#[derive(PartialEq, Eq, Hash)]
enum OutcomeScore {
    Win,
    Draw,
    Lose,
}

struct RpcSolver<'a> {
    hand_score: HashMap<&'a str, i32>,
    outcome_score: HashMap<OutcomeScore, i32>,
}

impl<'a> RpcSolver<'a> {
    fn build() -> Self {
        let mut hand_score = HashMap::new();
        hand_score.insert("X", 1);
        hand_score.insert("Y", 2);
        hand_score.insert("Z", 3);

        let mut outcome_score = HashMap::new();
        outcome_score.insert(OutcomeScore::Win, 6);
        outcome_score.insert(OutcomeScore::Draw, 3);
        outcome_score.insert(OutcomeScore::Lose, 0);

        Self {
            hand_score,
            outcome_score,
        }
    }

    fn get_score_for_part1(&self, opponent: &str, me: &str) -> i32 {
        match me {
            "X" => match opponent {
                "A" => {
                    return self.hand_score.get("X").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Draw).unwrap()
                }
                "B" => {
                    return self.hand_score.get("X").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Lose).unwrap()
                }
                "C" => {
                    return self.hand_score.get("X").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Win).unwrap()
                }
                _ => panic!("This hand is impossible!"),
            },
            "Y" => match opponent {
                "A" => {
                    return self.hand_score.get("Y").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Win).unwrap()
                }
                "B" => {
                    return self.hand_score.get("Y").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Draw).unwrap()
                }
                "C" => {
                    return self.hand_score.get("Y").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Lose).unwrap()
                }
                _ => panic!("This hand is impossible!"),
            },
            "Z" => match opponent {
                "A" => {
                    return self.hand_score.get("Z").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Lose).unwrap()
                }
                "B" => {
                    return self.hand_score.get("Z").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Win).unwrap()
                }
                "C" => {
                    return self.hand_score.get("Z").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Draw).unwrap()
                }
                _ => panic!("This hand is impossible!"),
            },
            _ => panic!("This hand is impossible!"),
        }
    }
    fn get_score_for_part2(&self, opponent: &str, outcome: &str) -> i32 {
        match opponent {
            "A" => match outcome {
                "X" => {
                    return self.hand_score.get("Z").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Lose).unwrap()
                }
                "Y" => {
                    return self.hand_score.get("X").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Draw).unwrap()
                }
                "Z" => {
                    return self.hand_score.get("Y").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Win).unwrap()
                }
                _ => panic!("This hand is impossible!"),
            },
            "B" => match outcome {
                "X" => {
                    return self.hand_score.get("X").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Lose).unwrap()
                }
                "Y" => {
                    return self.hand_score.get("Y").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Draw).unwrap()
                }
                "Z" => {
                    return self.hand_score.get("Z").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Win).unwrap()
                }
                _ => panic!("This hand is impossible! {opponent}, {outcome}"),
            },
            "C" => match outcome {
                "X" => {
                    return self.hand_score.get("Y").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Lose).unwrap()
                }
                "Y" => {
                    return self.hand_score.get("Z").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Draw).unwrap()
                }
                "Z" => {
                    return self.hand_score.get("X").unwrap()
                        + self.outcome_score.get(&OutcomeScore::Win).unwrap()
                }
                _ => panic!("This hand is impossible!"),
            },
            _ => panic!("This hand is impossible!"),
        }
    }
}
