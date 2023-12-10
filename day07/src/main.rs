use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut plays = read_plays(&contents);
    println!("Part 1: {}", part1(&mut plays));
    println!("Part 2: {}", part2(&mut plays));
}

fn read_plays(input: &str) -> Vec<Play> {
    input.split("\n").map(|p| {
        let mut iter = p.split_whitespace();
        let card_str = iter.next().unwrap();
        let bid_str = iter.next().unwrap();
        Play { cards: card_str.to_string(), bid: bid_str.parse::<u32>().unwrap() }
    }).collect()
}

#[derive(Debug)]
struct Play {
    cards: String,
    bid: u32,
}

impl Play {
    fn type_score(&self) -> usize {
        let mut m: HashMap<char, usize> = HashMap::new();
        for x in self.cards.chars() {
            *m.entry(x).or_default() += 1;
        }
        let max = m.iter().max_by_key(|(_, v)| *v).map(|(_, v)| *v).take().unwrap();
        let scores: Vec<&usize> = m.values().collect();
        match (scores.len(), max) {
            (1, _) => 7, // Five of a kind,
            (2, 4) => 6, // Four of a kind
            (2, 3) => 5, // Full house
            (3, 3) => 4, // Three of a kind
            (3, 2) => 3, // Two pair
            (4, _) => 2, // One pair
            (5, _) => 1, // High card
            _ => panic!("Impossible state: [{}], {}, {}", &self.cards, scores.len(), max)
        }
    }

    fn type_score_joker(&self) -> usize {
        let mut m: HashMap<char, usize> = HashMap::new();
        for x in self.cards.chars() {
            *m.entry(x).or_default() += 1;
        }

        let joker_count = m.get(&'J').or(Some(&0usize)).unwrap();
        let mut scores: Vec<&usize> = m.values().collect();
        scores.sort();
        let max = *scores[scores.len() - 1];
        match (scores.len(), max, joker_count) {
            (1, _, _) | (2, 4, 1) | (2, 3, 2) | (2, 3, 3) | (2, 4, 4) => 7, // Five of a kind
            (2, 4, 0) | (3, 3, 1) | (3, 2, 2) | (3, 3, 3) => 6, // Four of a kind
            (2, 3, 0) | (3, 2, 1) => 5, // Full house
            (3, 3, 0) | (4, 2, 1) | (4, 2, 2) => 4, // Three of a kind
            (3, 2, 0) => 3, // Two pair
            (4, 2, 0) | (5, 1, 1) => 2, // One pair
            (5, 1, 0) => 1, // High card
            _ => panic!("Impossible state: [{}] {}, {}, {}", &self.cards, scores.len(), max, joker_count)
        }
    }

    fn cmp(&self, other: &Play, card_sorting: &[char; 13]) -> Ordering {
        let chars: Vec<char> = self.cards.chars().collect();
        let other_chars: Vec<char> = other.cards.chars().collect();
        for i in 0..chars.len() {
            if chars[i] != other_chars[i] {
                let index = card_sorting.iter().position(|&r| r == chars[i]).unwrap();
                let index_other = card_sorting.iter().position(|&r| r == other_chars[i]).unwrap();
                return if index > index_other {
                    Ordering::Greater
                } else if index < index_other {
                    Ordering::Less
                } else {
                    panic!("Indexes can't be equal")
                };
            }
        }
        panic!("Impossible state: there's no equal cards")
    }
}

fn part1(plays: &mut Vec<Play>) -> u32 {
    let card_sorting = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    plays.sort_by(|p1, p2| {
        let p1_score = p1.type_score();
        let p2_score = p2.type_score();

        if p1_score > p2_score {
            Ordering::Greater
        } else if p1_score < p2_score {
            Ordering::Less
        } else {
            p1.cmp(p2, &card_sorting)
        }
    });

    let mut total: u32 = 0;
    for (idx, play) in plays.iter().enumerate() {
        total += (idx + 1) as u32 * play.bid
    }
    total
}

fn part2(plays: &mut Vec<Play>) -> u32 {
    let card_sorting = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
    plays.sort_by(|p1, p2| {
        let p1_score = p1.type_score_joker();
        let p2_score = p2.type_score_joker();

        if p1_score > p2_score {
            Ordering::Greater
        } else if p1_score < p2_score {
            Ordering::Less
        } else {
            p1.cmp(p2, &card_sorting)
        }
    });

    let mut total: u32 = 0;
    for (idx, play) in plays.iter().enumerate() {
        total += (idx + 1) as u32 * play.bid
    }
    total
}
