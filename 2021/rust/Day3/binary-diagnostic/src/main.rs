use std::env;

use io::loading;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let content = loading::read_file(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010",
        )
    });

    let report = DiagnosticReport::build(&content);
    let report = solve(&mut report).unwrap();

    println!("{report:?}");
    println!("Power consumption: {}", report.get_power_consumption());
}

fn solve(report: &mut DiagnosticReport) -> anyhow::Result<DiagnosticReport> {
    for i in 0..report.binary_numbers.first().unwrap().len() {
        let mut zero = 0;
        let mut one = 0;
        for binary_number in report.binary_numbers {
            let number = binary_number.get(i).unwrap();
            if number.eq(&0) {
                zero += 1
            } else {
                one += 1
            };
        }
        let gamma = if zero > one { '0' } else { '1' };
        let epsilon = if zero > one { '1' } else { '0' };
        report.gamma.push(gamma);
        report.epsilon.push(epsilon);
    }

    Ok(*report)
}

enum RatingKind {
    CarbonDioxideScrubber,
    OxygenScrubber,
}

#[derive(Debug)]
struct DiagnosticReport {
    binary_numbers: Vec<Vec<i32>>,
    gamma: String,
    epsilon: String,
}

impl DiagnosticReport {
    fn build(content: &str) -> DiagnosticReport {
        let gamma = String::from("");
        let epsilon = String::from("");

        DiagnosticReport {
            gamma,
            epsilon,
            binary_numbers: DiagnosticReport::to_nested_vec(content),
        }
    }

    fn to_nested_vec(content: &str) -> Vec<Vec<i32>> {
        let mut binary_numbers = Vec::<Vec<i32>>::new();
        for line in content.lines() {
            let mut binary_number = Vec::<i32>::new();
            for char in line.trim().chars() {
                binary_number.push(char.to_digit(10).unwrap().try_into().unwrap());
                // bit of a hack; take the char and convert it to digit in base 10. This returns an u32, hence the try_into()
            }
            binary_numbers.push(binary_number);
        }
        binary_numbers
    }

    fn get_power_consumption(&self) -> i32 {
        i32::from_str_radix(&self.gamma, 2).unwrap()
            * i32::from_str_radix(&self.epsilon, 2).unwrap()
    }

    fn get_life_support_rating(&self, rating: RatingKind) -> Vec<i32> {
        match rating {
            RatingKind::OxygenScrubber => {
                let bit_criteria = 1;
                let commons = &self.gamma;
                self.recursive_find(bit_criteria, commons, 0)
                    .get(0)
                    .unwrap()
            }
            RatingKind::CarbonDioxideScrubber => {
                let bit_criteria = 0;
                let commons = &self.epsilon;
                self.recursive_find(bit_criteria, commons, 0)
                    .get(0)
                    .unwrap()
            }
        }
    }

    fn recursive_find(&self, bit_criteria: i32, commons: &String, index: u32) -> Vec<Vec<i32>> {
        let tmp: i32 = commons
            .chars()
            .nth(index)
            .unwrap()
            .to_digit(10)
            .unwrap()
            .try_into()
            .unwrap();

        let result = self
            .binary_numbers
            .iter()
            .filter(|&n| *n.get(index).unwrap() == tmp)
            .collect();

        if result.len() != 1 {
            return self.recursive_find(bit_criteria, commons, index + 1);
        }
        result
    }
}
