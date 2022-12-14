use std::{collections::VecDeque, process};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    let mut solver = Solver::new();
    let answer = solver.solve(result);
    let n_char = solver.n;
    println!("Answer: {answer} after {n_char}");
}

struct Solver {
    window: VecDeque<char>,
    n: u32,
}

impl Solver {
    fn new() -> Solver {
        Solver {
            window: VecDeque::new(),
            n: 0,
        }
    }

    fn solve(&mut self, result: String) -> String {
        let chars = result.chars();
        for char in chars {
            self.window.push_back(char);
            self.n += 1;
            if self.window.len() == 14 {
                if !has_duplicate(&self.window) {
                    return self.window.clone().into_iter().collect();
                }
                self.window.pop_front();
            }
        }

        "".to_string()
    }
}

fn has_duplicate(queue: &VecDeque<char>) -> bool {
    for i in 0..queue.len() - 1 {
        let char_1 = queue[i];
        for j in 1 + i..queue.len() {
            let char_2 = queue[j];
            if char_1 == char_2 {
                return true;
            }
        }
    }

    false
}
