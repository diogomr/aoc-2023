use std::cmp::max;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");


    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(games: &str) -> i32 {
    let mut sum = 0;
    'outer: for game in games.split("\n") {
        let (game_id, plays) = game.split_once(":").unwrap();
        for play in plays.split(";") {
            for cube in play.split(",") {
                let (count, color) = cube.trim().split_once(" ").unwrap();
                let max = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    &_ => { panic!("Impossible state") }
                };
                if count.parse::<i32>().unwrap() > max {
                    continue 'outer;
                }
            }
        }
        let (_, id) = game_id.split_once(" ").unwrap();
        sum += id.parse::<i32>().unwrap();
    }
    sum
}

fn part2(games: &str) -> i32 {
    let mut sum = 0;
    for game in games.split("\n") {
        let (_, plays) = game.split_once(":").unwrap();
        let mut min_red = i32::MIN;
        let mut min_green = i32::MIN;
        let mut min_blue = i32::MIN;
        for play in plays.split(";") {
            for cube in play.split(",") {
                let (count, color) = cube.trim().split_once(" ").unwrap();
                match color {
                    "red" => min_red = max(min_red, count.parse::<i32>().unwrap()),
                    "green" => min_green = max(min_green, count.parse::<i32>().unwrap()),
                    "blue" => min_blue = max(min_blue, count.parse::<i32>().unwrap()),
                    &_ => { panic!("Impossible state") }
                };
            }
        }
        sum += min_red * min_green * min_blue;
    }
    sum
}
