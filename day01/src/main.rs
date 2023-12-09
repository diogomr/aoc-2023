use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");


    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.split("\n") {
        let mut first = -1;
        let mut last = -1;

        for char in line.chars() {
            match char.to_digit(10) {
                Some(number) => if first == -1 {
                    first = number as i32;
                    last = number as i32;
                } else {
                    last = number as i32;
                },
                None => {}
            }
        }

        let number = format!("{first}{last}").parse::<i32>()
            .expect("Failed parsing number");
        sum += number;
    }
    sum
}

fn part2(input: &str) -> i32 {
    let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut sum = 0;
    for line in input.split("\n") {
        let mut min = i32::MAX;
        let mut first: usize = 0;
        let mut max = i32::MIN;
        let mut last: usize = 0;
        for (pos, d) in digits.iter().enumerate() {
            for (index, _) in line.match_indices(*d).into_iter() {
                if (index as i32) < min {
                    min = index as i32;
                    first = pos % 10
                }
                if (index as i32) > max {
                    max = index as i32;
                    last = pos % 10
                }
            }
        }

        let number = format!("{first}{last}").parse::<i32>()
            .expect("Failed parsing number");

        sum += number;
    }
    sum
}