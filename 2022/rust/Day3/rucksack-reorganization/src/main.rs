fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        )
    });

    let lines = result.lines();

    let mut compartment_shared_items: Vec<char> = vec![];
    find_shared_item_within_compartments(lines.clone(), &mut compartment_shared_items);
    println!("Overlapping items: {compartment_shared_items:?}");
    let sum_compartments = calculate_total_priorities(compartment_shared_items);
    println!("The sum of priorties is: {sum_compartments}");

    let mut group_shared_items: Vec<char> = vec![];
    find_shared_item_within_groups(lines, &mut group_shared_items);
    println!("Overlapping items: {group_shared_items:?}");
    let sum_groups = calculate_total_priorities(group_shared_items);
    println!("The sum of priorties is: {sum_groups}");
}

fn calculate_total_priorities(shared_items: Vec<char>) -> u32 {
    let mut sum = 0;
    for char in &shared_items {
        if char.is_lowercase() {
            let priority_number = (*char as u32) - 96;
            sum += priority_number;
        } else {
            let priority_number = (*char as u32) - 38;
            sum += priority_number;
        }
    }

    sum
}

fn find_shared_item_within_compartments(lines: std::str::Lines, shared_items: &mut Vec<char>) {
    for line in lines {
        let char_vec: Vec<char> = line.chars().collect();
        let (first_compartement, second_compartment) = char_vec.split_at(char_vec.len() / 2);

        for char in first_compartement {
            if second_compartment.contains(char) {
                println!("char: {char}");
                shared_items.push(*char);
                println!("Vector: {shared_items:?}");
                break;
            }
        }
    }
}

fn find_shared_item_within_groups(lines: std::str::Lines, shared_items: &mut Vec<char>) {
    let mut group: Vec<Vec<char>> = vec![];
    for (i, line) in lines.into_iter().enumerate() {
        let char_vec: Vec<char> = line.chars().collect();
        println!("pushing {char_vec:?}");
        group.push(char_vec);

        if i % 3 == 2 {
            println!("Group: {group:?}");
            for char in group[0].iter() {
                if group[1].contains(char) && group[2].contains(char) {
                    println!("char: {char}");
                    shared_items.push(*char);
                    println!("Vector: {shared_items:?}");
                    break;
                }
            }
            group.clear();
        }
    }
}
