fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        String::from(
            "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
        )
    });

    // todo: io::loading:: contains a lines_to_vec_u32 function. Add a function lines_to_vec_u32_grouped that returns a vec of vec grouped on an empty line
    let lines = result.lines();
    let mut sum = 0;
    let mut highest_sum: Vec<i32> = vec![0, 0, 0];

    for line in lines {
        if line == "" {
            if sum > highest_sum[0] {
                highest_sum[0] = sum;
            }
            sum = 0;
            highest_sum.sort();
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }

    println!("Highest calories: {highest_sum:?}");
    let total: i32 = highest_sum.iter().sum();
    println!("Total: {total}");
}
