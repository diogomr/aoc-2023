use std::collections::{HashMap, HashSet};
use std::fs;

use prime_factorization::Factorization;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(input: &str) -> usize {
    let (inst, network) = input.split_once("\n\n").unwrap();

    let network_map = build_network_map(network);


    let instructions: Vec<char> = inst.chars().collect();
    let mut res = 0;
    let mut cur_node = "AAA";
    loop {
        let dir = instructions[res % instructions.len()];
        res += 1;
        let jump = network_map.get(cur_node).unwrap();
        if dir == 'L' {
            cur_node = &jump.0
        } else {
            cur_node = &jump.1
        }
        if cur_node == "ZZZ" {
            break;
        }
    }
    res
}

fn part2(input: &str) -> u64 {
    let (inst, network) = input.split_once("\n\n").unwrap();
    let network_map = build_network_map(network);


    let instructions: Vec<char> = inst.chars().collect();
    let mut cur_nodes: Vec<&String> = network_map.keys().filter(|n| n.ends_with("A")).collect();
    let mut ins_res: Vec<u64> = vec![0; cur_nodes.len()];
    let mut res = 0;
    loop {
        let dir = instructions[res % instructions.len()];
        res += 1;
        let mut new_nodes: Vec<&String> = Vec::new();
        for cur in cur_nodes {
            let jump = network_map.get(cur).unwrap();
            if dir == 'L' {
                new_nodes.push(&jump.0)
            } else {
                new_nodes.push(&jump.1)
            }
        }

        for (idx, nn) in new_nodes.iter().enumerate() {
            if ins_res[idx] == 0 && nn.ends_with("Z") {
                ins_res[idx] = res as u64
            }
        }

        if ins_res.iter().all(|n| *n != 0) {
            break;
        }

        cur_nodes = new_nodes;
    }

    let mut prime_factors: HashSet<u64> = HashSet::new();
    for res in &ins_res {
        let factors = Factorization::run(*res).factors;
        for factor in &factors {
            prime_factors.insert(*factor);
        }
    }
    // The LCM is calculated by picking the unique primes from each result and multiplying them together
    prime_factors.iter().fold(1, |acc, prime| acc * prime)
}

fn build_network_map(network: &str) -> HashMap<String, (String, String)> {
    let mut network_map: HashMap<String, (String, String)> = HashMap::new();

    let net_nodes = network.split("\n");
    for net_node in net_nodes {
        let mut node = "";
        let mut left = "";
        let mut right = "";
        for cap in Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap().captures(net_node).unwrap().iter() {
            let capture = cap.unwrap();
            if capture.as_str() == net_node { continue; }
            if node.is_empty() {
                node = capture.as_str()
            } else if left.is_empty() {
                left = capture.as_str()
            } else if right.is_empty() {
                right = capture.as_str()
            }
        }
        network_map.insert(node.parse().unwrap(), (left.parse().unwrap(), right.parse().unwrap()));
    }
    network_map
}