use std::fs;
use std::cmp;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read input.txt");
    let lines = input.lines();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut valid_games = Vec::new();
    let mut total_power = 0;

    for line in lines {
        let id = get_id(line);
        let rounds = get_cubes(line);

        let (min_red, min_green, min_blue) = get_minimum_cubes(&rounds);
        let power = min_red * min_green * min_blue;
        total_power += power;

        if is_valid_game(&rounds, max_red, max_green, max_blue) {
            if let Some(id) = id {
                valid_games.push(id);
            }
        }
    }
    let valid_games: i32 = valid_games.iter().sum();
    println!("Sum of valid game ids: {}", valid_games);
    println!("Total Power: {}", total_power);
}

fn get_id(line: &str) -> Option<i32> {
    line.strip_prefix("Game ").and_then(|s| s.split(':').next()).and_then(|s| s.trim().parse().ok())
}

fn get_cubes(line: &str) -> Vec<Vec<(i32, String)>> {
    line.split(':')
        .nth(1)
        .unwrap_or("")
        .split(';')
        .map(|round| {
            round.trim().split(',')
                .filter_map(|part| {
                    let mut iter = part.trim().split_whitespace();
                    let count = iter.next()?.parse().ok()?;
                    let color = iter.next()?.to_string();
                    Some((count, color))
                })
            .collect()
        })
    .collect()
}

fn get_minimum_cubes(rounds: &Vec<Vec<(i32, String)>>) -> (i32, i32, i32) {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

    for round in rounds {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for (count, color) in round {
            match color.as_str() {
                "red" => red_count += count,
                "green" => green_count += count,
                "blue" => blue_count += count,
                _ => {}
            }
        }

        min_red = cmp::max(min_red, red_count);
        min_green = cmp::max(min_green, green_count);
        min_blue = cmp::max(min_blue, blue_count);
    }

    (min_red, min_green, min_blue)
}

fn is_valid_game(rounds: &Vec<Vec<(i32, String)>>, red: i32, green: i32, blue: i32) -> bool {
    for round in rounds {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        for (count, color) in round {
            match color.as_str() {
                "red" => red_count += count,
                "green" => green_count += count,
                "blue" => blue_count += count,
                _ => {}
            }
        }

        if red_count > red || green_count > green || blue_count > blue {
            return false;
        }
    }

    true
}
