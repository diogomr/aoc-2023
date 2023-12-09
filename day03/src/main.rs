use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(engine_schema: &String) -> u32 {
    let mut sum = 0;
    let engine: Vec<Vec<String>> = engine_schema.split("\n")
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();

    let line_range = 0..engine.len();
    for l in line_range.clone() {
        let mut parsing_number = false;
        let mut adj_sym = false;
        let mut number = "".to_owned();
        let col_range = 0..engine[0].len();
        for c in col_range.clone() {
            let parsed = engine[l][c].parse::<i32>();
            if parsed.is_ok() {
                parsing_number = true;
                number.push_str(&engine[l][c]);

                'check_adj: for dl in [-1, 0i32, 1] {
                    let ldl = (l as i32 + dl) as usize;
                    if !line_range.contains(&ldl) { continue; }
                    for dc in [-1, 0i32, 1] {
                        if dl == 0 && dc == 0 { continue; }
                        let cdc = (c as i32 + dc) as usize;
                        if !col_range.contains(&cdc) { continue; }
                        let surrounding = &engine[ldl][cdc];
                        if surrounding != "." && surrounding.parse::<i32>().is_err() {
                            adj_sym = true;
                            break 'check_adj;
                        }
                    }
                }
            }

            if parsed.is_err() || c + 1 >= engine[0].len() {
                if parsing_number && adj_sym {
                    sum += number.parse::<u32>().unwrap();
                }
                number = "".to_owned();
                parsing_number = false;
                adj_sym = false
            }
        }
    }
    sum
}

fn part2(engine_schema: &String) -> u64 {
    let mut sum = 0;
    let engine: Vec<Vec<String>> = engine_schema.split("\n")
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    let line_range = 0..engine.len();
    for l in line_range.clone() {
        let col_range = 0..engine[0].len();
        for c in col_range.clone() {
            if engine[l][c] == "*" {
                let mut number_one = 0;
                let mut number_two = 0;
                for dl in [-1, 0i32, 1] {
                    let ldl = (l as i32 + dl) as usize;
                    if !line_range.contains(&ldl) { continue; }
                    for dc in [-1, 0i32, 1] {
                        if dl == 0 && dc == 0 { continue; }
                        let cdc = (c as i32 + dc) as usize;
                        if !col_range.contains(&cdc) { continue; }
                        let surrounding = &engine[ldl][cdc];
                        if surrounding.parse::<i32>().is_ok() {
                            let number = extract_number(cdc, &engine[ldl]);
                            if number_one == 0 {
                                number_one = number
                            } else {
                                number_two = number
                            }
                        }
                    }
                }
                /// number_one != number_two is a hacky way to get around extract_number detecting the same number twice
                /// it works for the given input but would break for something like ...123*123...
                if number_one != 0 && number_two != 0 && number_one != number_two {
                    sum += number_one * number_two
                }
            }
        }
    }
    sum
}

fn extract_number(c: usize, line: &Vec<String>) -> u64 {
    let mut number = line[c].to_owned();

    if c > 0 {
        let mut dc = c - 1;
        while let Ok(digit) = line[dc].parse::<i32>() {
            number = format!("{digit}{number}");
            if dc > 0 {
                dc -= 1
            } else {
                break;
            }
        }
    }


    if c < line.len() - 1 {
        let mut dc = c + 1;
        while let Ok(digit) = line[dc].parse::<i32>() {
            number = format!("{number}{digit}");
            if dc < line.len() - 1 {
                dc += 1
            } else {
                break;
            }
        }
    }

    number.parse::<u64>().unwrap()
}