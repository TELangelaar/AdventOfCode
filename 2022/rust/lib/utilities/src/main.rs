use std::{env, process};

use crate::io::loading;
pub mod io;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let result = loading::read_file(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("{result}");
}
