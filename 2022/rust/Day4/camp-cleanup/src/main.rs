fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
        )
    });

    let lines = result.lines();

    let mut overlapping_pairs = 0;
    for line in lines {
        let parts = line.split(",").collect::<Vec<&str>>();

        let mut pair = SectionRangePair::new();
        for r in parts {
            let range: Vec<u32> = r
                .split("-")
                .map(|c| c.parse::<u32>().unwrap_or_else(|e| panic!("{e}")))
                .collect();
            let sr = SectionRange::build(range);
            pair.add(sr);
        }

        let answ = pair.solve_part2();
        if answ == 1 {
            println!("Overlapping pair: {pair:?}");
            overlapping_pairs += answ;
        }
        pair.clear();
    }

    println!("Number of overlapping pairs: {overlapping_pairs}");
}

#[derive(Debug)]
struct SectionRangePair {
    first: Option<SectionRange>,
    second: Option<SectionRange>,
}

impl SectionRangePair {
    fn new() -> SectionRangePair {
        SectionRangePair {
            first: None,
            second: None,
        }
    }
    fn add(&mut self, section_range: SectionRange) {
        match self.first {
            Some(_) => self.second = Some(section_range),
            None => self.first = Some(section_range),
        }
    }

    fn clear(&mut self) {
        self.first = None;
        self.second = None;
    }

    fn solve_part1(&mut self) -> u32 {
        self.sort_by_first();
        let first = self.first.as_ref().unwrap();
        let second = self.second.as_ref().unwrap();

        if first.low == second.low {
            1
        } else if first.low < second.low && second.high <= first.high {
            1
        } else {
            0
        }
    }

    fn solve_part2(&mut self) -> u32 {
        self.sort_by_first();
        let first = self.first.as_ref().unwrap();
        let second = self.second.as_ref().unwrap();

        if first.high < second.low {
            0
        } else {
            1
        }
    }

    fn sort_by_first(&mut self) {
        let first = self.first.unwrap();
        let second = self.second.unwrap();

        if first.low >= second.low {
            self.first = Some(second);
            self.second = Some(first);
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SectionRange {
    low: u32,
    high: u32,
}

impl SectionRange {
    fn build(range: Vec<u32>) -> SectionRange {
        SectionRange {
            low: range.iter().min().unwrap().to_owned(),
            high: range.iter().max().unwrap().to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{SectionRange, SectionRangePair};

    #[test]
    fn solve_shouldreturn_zero_whenpairsare_notoverlapping() {
        let sr_1 = SectionRange::build(vec![4, 90]);
        let sr_2 = SectionRange::build(vec![1, 4]);

        let mut srp = SectionRangePair::new();
        srp.add(sr_1);
        srp.add(sr_2);

        assert_eq!(0, srp.solve_part1());
    }

    #[test]
    fn solve_shouldreturn_zero_whenpairsare_notoverlapping_2() {
        let sr_1 = SectionRange::build(vec![60, 90]);
        let sr_2 = SectionRange::build(vec![20, 30]);

        let mut srp = SectionRangePair::new();
        srp.add(sr_1);
        srp.add(sr_2);

        assert_eq!(0, srp.solve_part1());
    }

    #[test]
    fn solve_shouldreturn_one_whenpairsare_overlapping() {
        let sr_1 = SectionRange::build(vec![1, 90]);
        let sr_2 = SectionRange::build(vec![1, 4]);

        let mut srp = SectionRangePair::new();
        srp.add(sr_1);
        srp.add(sr_2);

        assert_eq!(1, srp.solve_part1());
    }

    #[test]
    fn solve_shouldreturn_one_whenpairsare_overlapping_2() {
        let sr_1 = SectionRange::build(vec![1, 90]);
        let sr_2 = SectionRange::build(vec![2, 4]);

        let mut srp = SectionRangePair::new();
        srp.add(sr_1);
        srp.add(sr_2);

        assert_eq!(1, srp.solve_part1());
    }

    #[test]
    fn solve_shouldreturn_one_whenpairsare_overlapping_3() {
        let sr_1 = SectionRange::build(vec![4, 8]);
        let sr_2 = SectionRange::build(vec![3, 8]);

        let mut srp = SectionRangePair::new();
        srp.add(sr_1);
        srp.add(sr_2);

        assert_eq!(1, srp.solve_part1());
    }
}
