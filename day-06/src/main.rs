use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");


    let part1 = part1(&input);
    let part2 = part2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<i32> = lines[0].split_whitespace().skip(1).map(|t| t.parse().unwrap()).collect();
    let distances: Vec<i32> = lines[1].split_whitespace().skip(1).map(|d| d.parse().unwrap()).collect();


    times.iter().zip(distances.iter())
        .map(|(&time, &record)| {
            let mut possible_ways = 0;
            for hold_time in 0..time {
                let distance = hold_time * (time - hold_time);
                if distance > record {
                    possible_ways += 1;
                }
            }
            possible_ways
        })
    .product()
}

fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let time: i64 = lines[0].split_whitespace().skip(1).collect::<String>().parse().unwrap();
    let distance: i64 = lines[1].split_whitespace().skip(1).collect::<String>().parse().unwrap();

    let mut possible_ways = 0;
    for hold_time in 0..time {
        let travelled_distance = hold_time * (time - hold_time);
        if travelled_distance > distance {
            possible_ways += 1;
        }
    }

    possible_ways
}
