use std::fs;

#[derive(Debug)]
struct MapRule {
    destination: i64,
    source: i64,
    length: i64,
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    let part1 = part1(&input);
//    let part2 = part2(&input);

    println!("Part 1: {}", part1);
//    println!("Part 2: {}", part2);
}


fn part1(input: &str) -> i64 {
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();
    let seeds_str = seeds_str.strip_prefix("seeds: ").unwrap();
    let seeds: Vec<i64> = seeds_str.split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    let mut maps = Vec::new();

    for map in maps_str.split("\n\n") {
        let (_, rules) = map.split_once("\n").unwrap();
        let mut map = Vec::new();
        for line in rules.lines() {
            let parts: Vec<i64> = line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            let destination = parts[0];
            let source = parts[1];
            let length = parts[2];
            map.push(MapRule {
                destination,
                source,
                length,
            });
        }
        maps.push(map);
    }

    let mut min = i64::MAX;

    for seed in seeds {
        let mut current = seed;

        'maps: for map in &maps {
            for rule in map {
                if current >= rule.source && current < rule.source + rule.length {
                    let offset = current - rule.source;
                    current = rule.destination + offset;
                    continue 'maps;
                }
            }
        }
        min = min.min(current);
    }

    min
}

// fn part2(input: &str) -> i64 { }
