use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    let cards: Vec<&str> = input.lines().collect();

    let total_points = part1(&cards);
    let total_cards = part2(&cards);

    println!("Total points: {}", total_points);
    println!("Total cards: {}", total_cards);
}

fn part1(cards: &[&str]) -> i32 {
    let mut total_points = 0;

    for card in cards {
        let parts: Vec<&str> = card.split(": ").collect();
        let winning_numbers: Vec<i32> = parts[1]
            .split('|')
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        let player_numbers: Vec<i32> = parts[1]
            .split('|')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        let mut points = 0;
        for number in player_numbers {
            if winning_numbers.contains(&number) {
                if points == 0 { points = 1 } 
                else { points *= 2 }
            }
        }

        total_points += points;
    }


    total_points
}

fn part2(cards: &[&str]) -> usize {
    let mut total_cards = cards.len();
    let mut cards_to_process: Vec<(usize, Vec<i32>, Vec<i32>)> = Vec::new();

    for (index, card) in cards.iter().enumerate() {
        let parts: Vec<&str> = card.splitn(2, ": ").nth(1).unwrap().split('|').collect();
        let winning_numbers: Vec<i32> = parts[0]
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();
        let player_numbers: Vec<i32> = parts[1]
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        cards_to_process.push((index, winning_numbers, player_numbers));
    }

    // process initial cards and any additional cards won
    while let Some((index, winning_numbers, player_numbers)) = cards_to_process.pop() {
        let mut matches = 0;
        for number in player_numbers {
            if winning_numbers.contains(&number) {
                matches += 1;
            }
        }

        for i in 0..matches {
            if index + i + 1 < cards.len() {
                let next_card = cards[index + i + 1];
                let parts: Vec<&str> = next_card.split(": ").collect();
                let next_winning_numbers: Vec<i32> = parts[1]
                    .split('|')
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();
                let next_player_numbers: Vec<i32> = parts[1]
                    .split('|')
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();

                cards_to_process.push((index + i + 1, next_winning_numbers, next_player_numbers));
                total_cards += 1
            }
        }
    }

    total_cards
}
