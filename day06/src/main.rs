use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(input: &str) -> u32 {
    let (time_input, distance_input) = input.split_once("\n").unwrap();
    let (_, times_raw) = time_input.split_once(":").unwrap();
    let (_, distance_raw) = distance_input.split_once(":").unwrap();
    let times: Vec<u32> = times_raw.split_whitespace().map(|str| str.parse::<u32>().unwrap()).collect();
    let distances: Vec<u32> = distance_raw.split_whitespace().map(|str| str.parse::<u32>().unwrap()).collect();

    let mut results: Vec<u32> = vec![0; times.len()];
    for race in 0..times.len() {
        for hold in 1..times[race] {
            let speed = hold;
            let rem_duration = times[race] - hold;
            let dist = speed * rem_duration;
            if dist > distances[race] {
                results[race] += 1;
            }
        }
    }

    results.iter().fold(1, |acc, r| acc * r)
}

fn part2(input: &str) -> u64 {
    let (time_input, distance_input) = input.split_once("\n").unwrap();
    let (_, times_raw) = time_input.split_once(":").unwrap();
    let (_, distance_raw) = distance_input.split_once(":").unwrap();
    let time = times_raw.replace(" ", "").parse::<u64>().unwrap();
    let distance = distance_raw.replace(" ", "").parse::<u64>().unwrap();

    let mut count: u64 = 0;

    for hold in 1..time {
        let speed = hold;
        let rem_duration = time - hold;
        let dist = speed * rem_duration;
        if dist > distance {
            count += 1;
        }
    }

    count
}