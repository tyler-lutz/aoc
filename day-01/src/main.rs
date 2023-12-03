use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_digits(input: &str) -> (Option<i32>, Option<i32>) {
    let mut first_digit = None;
    let mut last_digit = None;

    let modified = input.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero");

    for char in modified.chars() {
        if char.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(char.to_digit(10).unwrap() as i32);
            }
            last_digit = Some(char.to_digit(10).unwrap() as i32);
        }
    }

    (first_digit, last_digit)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        for line in lines {
            if let Ok(input) = line {
                if let (Some(first), Some(last)) = get_digits(&input) {
                    let number = format!("{}{}", first, last)
                        .parse::<i32>()
                        .unwrap();
                    sum += number;
                }
            }
        }
        println!("Sum: {}", sum);
    }
}
