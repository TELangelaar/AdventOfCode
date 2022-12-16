use nom::{
    bytes::complete::take,
    character::complete::line_ending,
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

fn main() {
    let args = &vec!["input.txt".to_string()];

    let result = io::loading::read_file(args).unwrap_or_else(|err| {
        eprintln!("{err}");

        String::from(
            "\
30373
25512
65332
33549
35390",
        )
    });

    let grid = grid(&result).unwrap().1;
    println!("{grid:?}");

    let visible_trees = solve_part1(&grid);
    println!("Visible trees from outside grid: {visible_trees}");

    let scenic_score: Vec<u32> = solve_part2(&grid);
    let highest_score = scenic_score.iter().max().unwrap();
    println!("Highest scenic_score: {highest_score}");
}

fn solve_part2(grid: &Vec<Vec<u8>>) -> Vec<u32> {
    let mut scenic_scores: Vec<u32> = vec![];
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            let tree = grid[i][j];
            let mut scenic_score = vec![];
            scenic_score.push(calculate_viewing_distance_up(&grid, tree, i, j));
            scenic_score.push(calculate_viewing_distance_down(&grid, tree, i, j));
            scenic_score.push(calculate_viewing_distance_left(&grid[i], tree, j));
            scenic_score.push(calculate_viewing_distance_right(&grid[i], tree, j));
            println!("tree {tree} with {scenic_score:?}");
            scenic_scores.push(scenic_score.iter().fold(1, |acc, x| acc * x));
        }
    }

    println!("{scenic_scores:?}");
    scenic_scores
}

fn calculate_viewing_distance_up(
    grid: &Vec<Vec<u8>>,
    tree: u8,
    from_row: usize,
    col: usize,
) -> u32 {
    let mut visible_trees = 0;

    for i in (0..=from_row - 1).rev() {
        if grid[i][col] < tree {
            visible_trees += 1;
        } else {
            visible_trees += 1;
            break;
        }
    }

    visible_trees
}

fn calculate_viewing_distance_down(
    grid: &Vec<Vec<u8>>,
    tree: u8,
    from_row: usize,
    col: usize,
) -> u32 {
    let mut visible_trees = 0;
    for i in from_row + 1..grid.len() {
        if grid[i][col] < tree {
            visible_trees += 1;
        } else {
            visible_trees += 1;
            break;
        }
    }

    visible_trees
}

fn calculate_viewing_distance_left(grid: &Vec<u8>, tree: u8, col: usize) -> u32 {
    let mut visible_trees = 0;
    for j in (0..col).rev() {
        if grid[j] < tree {
            visible_trees += 1;
        } else {
            visible_trees += 1;
            break;
        }
    }

    visible_trees
}

fn calculate_viewing_distance_right(grid: &Vec<u8>, tree: u8, col: usize) -> u32 {
    let mut visible_trees = 0;
    for j in col + 1..grid.len() {
        if grid[j] < tree {
            visible_trees += 1;
        } else {
            visible_trees += 1;
            break;
        }
    }

    visible_trees
}

fn solve_part1(grid: &Vec<Vec<u8>>) -> usize {
    let col_length = grid.len();
    let row_length = grid[0].len();
    let mut visible_trees = 4 * col_length - 4;
    // outside trees are always visible
    for i in 1..col_length - 1 {
        for j in 1..row_length - 1 {
            let tree = &grid[i][j];

            let mut visible = true;

            for jj in 0..j {
                let tree_left = &grid[i][jj];
                if tree_left >= tree {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_trees += 1;
                continue;
            } else {
                visible = true;
            }

            for jj in j + 1..row_length {
                let tree_right = &grid[i][jj];
                if tree_right >= tree {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_trees += 1;
                continue;
            } else {
                visible = true;
            }

            for ii in 0..i {
                let tree_top = &grid[ii][j];
                if tree_top >= tree {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_trees += 1;
                continue;
            } else {
                visible = true;
            }

            for ii in i + 1..col_length {
                let tree_bottom = &grid[ii][j];
                if tree_bottom >= tree {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn tree(input: &str) -> IResult<&str, u8> {
    let (input, tree) = map_res(take(1usize), |s: &str| s.parse::<u8>())(input)?;

    Ok((input, tree))
}

fn grid(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    let (input, grid) = separated_list1(line_ending, many1(tree))(input)?;

    Ok((input, grid))
}
