use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    let schematic: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let sum_of_parts = part1(&schematic);
    let sum_of_gear_ratios = part2(&schematic);

    println!("Sum of part numbers: {}", sum_of_parts);
    println!("Sum of gear ratios: {}", sum_of_gear_ratios);
}

fn part1(schematic: &Vec<Vec<char>>) -> i32 {
    let mut sum_of_parts = 0;

    let mut processed = vec![vec![false; schematic[0].len()]; schematic.len()];

    for (i, row) in schematic.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if !processed[i][j] && char.is_digit(10) && is_adjacent_to_symbol(schematic, i, j) {
                sum_of_parts += get_part_number(schematic, i, j, &mut processed);
            }
        }
    }

    sum_of_parts
}

fn part2(schematic: &Vec<Vec<char>>) -> i32 {
    let mut sum_of_gear_ratios = 0;

    for (i, row) in schematic.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char == '*' {
                if let Some(gear_ratio) = calculate_gear_ratio(schematic, i, j) {
                    sum_of_gear_ratios += gear_ratio;
                }
            }
        }
    }

    sum_of_gear_ratios
}

fn calculate_gear_ratio(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> Option<i32> {
    let part_numbers = get_adjacent_part_numbers(&schematic, i, j);

    if part_numbers.len() == 2 {
        Some(part_numbers[0] * part_numbers[1])
    } else {
        None
    }
}

fn get_adjacent_part_numbers(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<i32> {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut processed = vec![vec![false; schematic[0].len()]; schematic.len()];

    let mut part_numbers = Vec::new();

    for (di, dj) in directions.iter() {
        let i = i as i32 + di;
        let j = j as i32 + dj;

        if i >= 0 && i < schematic.len() as i32 && j >= 0 && j < schematic[0].len() as i32 {
            let adjacent_char = schematic[i as usize][j as usize];
            if !processed[i as usize][j as usize] && adjacent_char.is_digit(10) {
                let part_number = get_part_number(schematic, i as usize, j as usize, &mut processed);
                part_numbers.push(part_number);
            }
        }
    }

    part_numbers
}

fn is_adjacent_to_symbol(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let row_len = schematic[0].len();
    let col_len = schematic.len();

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    for (di, dj) in directions.iter() {
        let i = i as i32 + di;
        let j = j as i32 + dj;

        if i >= 0 && i < col_len as i32 && j >= 0 && j < row_len as i32 {
            let adjacent_char = schematic[i as usize][j as usize];
            if !adjacent_char.is_digit(10) && adjacent_char != '.' {
                return true;
            }
        }
    }

    false
}

fn get_part_number(schematic: &Vec<Vec<char>>, i: usize, j: usize, processed: &mut Vec<Vec<bool>>) -> i32 {
    let row = &schematic[i];
    let mut part_number = String::new();
    let mut k = j;

    // Search for start of number
    while k > 0 && row[k-1].is_digit(10) {
        k -= 1;
    }
    
    // Build the number from left to right
    while k < row.len() && row[k].is_digit(10) {
        part_number.push(row[k]);
        processed[i][k] = true;
        k += 1;
    }

    part_number.parse::<i32>().unwrap_or(0)
}
