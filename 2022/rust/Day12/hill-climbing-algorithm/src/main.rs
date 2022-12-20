fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        )
    });
}
