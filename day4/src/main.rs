/**
Reads the input file into a 2d vector of chars
**/
fn read_input() -> Vec<Vec<char>> {
    let mut input = Vec::new();
    let mut line = String::new();
    std::fs::read_to_string("src/input.txt").unwrap().lines().for_each(|l| {
        line = l.to_string();
        input.push(line.chars().collect());
    });
    input
}

/**
Count the number of adjacent rolls of paper in the input in all 8 directions
**/
fn count_adjacent_rolls(row: u32, col: u32, input: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            // skip current one
            if i == 0 && j == 0 {
                continue;
            }
            let new_row = row as i32 + i;
            let new_col = col as i32 + j;
            // skip out of bounds
            if new_row < 0 || new_row >= input.len() as i32 || new_col < 0 || new_col >= input[0].len() as i32 {
                continue;
            }
            if input[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

/**
Whether a roll of paper can be accessed from the given position.
A roll of paper can only be accessed if there are fewer than 4 rolls of paper in the adjacent positions
**/
fn can_access(row: u32, col: u32, input: &Vec<Vec<char>>) -> bool {
    count_adjacent_rolls(row, col, input) < 4
}

fn is_roll(row: u32, col: u32, input: &Vec<Vec<char>>) -> bool {
    input[row as usize][col as usize] == '@'
}

fn remove_accessible_rolls(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut accessible_rolls = 0;
    let mut new_input = input.clone();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if is_roll(i as u32, j as u32, &input) && can_access(i as u32, j as u32, &input) {
                accessible_rolls += 1;
                new_input[i][j] = '.';
            }
        }
    }
    //println!("Accessible rolls removed: {}", accessible_rolls);
    new_input
}

fn are_any_rolls_accessible(input: &Vec<Vec<char>>) -> bool {
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if is_roll(i as u32, j as u32, input) && can_access(i as u32, j as u32, &input) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut input = read_input();
    let mut accessible_rolls = 0;
    let mut steps = 0;
    while are_any_rolls_accessible(&input) {
        steps += 1;
        // count accessible rolls
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if is_roll(i as u32, j as u32, &input) && can_access(i as u32, j as u32, &input) {
                    accessible_rolls += 1;
                }
            }
        }
        println!("Accessible rolls in run: {}", accessible_rolls);
        input = remove_accessible_rolls(input);
    }
    
    println!("Accessible rolls: {}", accessible_rolls);
    println!("Steps: {}", steps);
}
