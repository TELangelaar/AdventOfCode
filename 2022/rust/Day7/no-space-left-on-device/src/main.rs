#![feature(iter_intersperse)]
use std::{collections::BTreeMap, process};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        )
    });

    let operations = operations(&result).unwrap().1;

    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<&str> = vec![];

    for operation in operations.iter() {
        match operation {
            Operation::Cd(Cd::Root) => {
                context.push("");
            }
            Operation::Cd(Cd::Up) => {
                context.pop();
            }
            Operation::Cd(Cd::Down(dir_name)) => {
                context.push(dir_name);
            }
            Operation::Ls(file_systems) => {
                //add file_systems to the directories map with the full path name as its key
                directories
                    .entry(context.iter().cloned().intersperse("/").collect::<String>())
                    .or_insert(vec![]);
                for file in file_systems {
                    match file {
                        FileSystem::File { size } => {
                            directories
                                .entry(context.iter().cloned().intersperse("/").collect::<String>())
                                .and_modify(|vec| vec.push(File { size: *size }));
                        }
                        FileSystem::Dir(_) => (),
                    }
                }
            }
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path.split("/").collect::<Vec<&str>>();

        //sum the file sizes of all the files the directory contains
        let size = files.iter().map(|File { size }| size).sum::<u32>();

        for i in 0..dirs.len() {
            sizes
                .entry(
                    (&dirs[0..=i])
                        .iter()
                        .cloned()
                        .intersperse("/")
                        .collect::<String>(),
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    let answer = solve_part1(&sizes);
    println!("{answer}");

    let candidates = solve_part2(&sizes);
    match candidates {
        Some(size) => println!("answer: {size}"),
        None => println!("Didn't find a directory to delete"),
    }
}

fn solve_part2(sizes: &BTreeMap<String, u32>) -> Option<&u32> {
    let total_space = 70_000_000;
    let total_unused_space_needed = 30_000_000;
    let current_unused_space = total_space - sizes.get("").unwrap().clone();
    let to_be_deleted_space = total_unused_space_needed - current_unused_space;
    let candidates = sizes
        .iter()
        .filter(|(_, &size)| size >= to_be_deleted_space)
        .map(|(_, size)| size)
        .min();
    candidates
}

fn solve_part1(sizes: &BTreeMap<String, u32>) -> String {
    //select those directories that have a total size of less than 100_000 and sum their size together.
    sizes
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string()
}

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<FileSystem<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum FileSystem<'a> {
    File { size: u32 },
    Dir(&'a str),
}

#[derive(Debug)]
struct File {
    size: u32,
}

fn file(input: &str) -> IResult<&str, FileSystem> {
    let (input, (size, _)) =
        separated_pair(complete::u32, tag(" "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;

    Ok((input, FileSystem::File { size }))
}

fn directory(input: &str) -> IResult<&str, FileSystem> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, FileSystem::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, filesystems) = separated_list1(line_ending, alt((file, directory)))(input)?;

    Ok((input, Operation::Ls(filesystems)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, cd_direction) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match cd_direction {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };

    Ok((input, op))
}

fn operations(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, commands) = separated_list1(line_ending, alt((cd, ls)))(input)?;

    Ok((input, commands))
}
