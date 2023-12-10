use std::collections::VecDeque;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(input: &str) -> u64 {
    let cards: Vec<&str> = input.split("\n").collect();
    let mut res: u64 = 0;
    for card in &cards {
        let (_, numbers) = card.split_once(":").unwrap();
        let (winning, got) = numbers.split_once("|").unwrap();
        let winning_numbers: Vec<&str> = winning.trim().split(" ")
            .filter_map(|n| match n.trim() {
                "" => None,
                number => Some(number)
            })
            .collect();
        let got_numbers: Vec<&str> = got.trim().split(" ")
            .filter_map(|n| match n.trim() {
                "" => None,
                number => Some(number)
            })
            .collect();
        let count = winning_numbers.iter()
            .fold(0, |acc, e| acc + if got_numbers.contains(e) { 1 } else { 0 });

        if count > 0 {
            res += 2u64.pow(count - 1)
        }
    }
    res
}

fn part2(input: &str) -> i32 {
    let mut cards: VecDeque<&str> = input.split("\n").collect();
    let original = cards.clone();
    let mut card_count = 0;
    while let Some(card) = &cards.pop_front() {
        let (_, numbers) = card.split_once(":").unwrap();
        let (winning, got) = numbers.split_once("|").unwrap();
        let winning_numbers: Vec<&str> = winning.trim().split(" ")
            .filter_map(|n| match n.trim() {
                "" => None,
                number => Some(number)
            })
            .collect();
        let got_numbers: Vec<&str> = got.trim().split(" ")
            .filter_map(|n| match n.trim() {
                "" => None,
                number => Some(number)
            })
            .collect();
        let count = winning_numbers.iter()
            .fold(0, |acc, e| acc + if got_numbers.contains(e) { 1 } else { 0 });
        let original_idx = original.iter().position(|r| r == card).unwrap() as i32;
        for d in 1..=count {
            cards.push_back(original[(original_idx + d) as usize])
        }
        card_count += 1;
    }
    card_count
}