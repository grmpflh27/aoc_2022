use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::FromIterator;

const DAY: &str = "day4";

fn main() {
    let file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let sections = read_lines(&file_path);
    part1(&sections);
    part2(&sections);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    println!("Reading {}", abs_filename);
    let contents = fs::read_to_string(abs_filename).expect("Something went wrong reading the file");

    return contents;
}

fn part1(sections: &str) {
    let mut overlap_pairs: u32 = 0;
    for section in sections.lines() {
        if full_overlaps(section) {
            overlap_pairs += 1;
        }
    }

    println!("part1: {}", overlap_pairs);
}

fn full_overlaps(section: &str) -> bool {
    let ranges: Vec<&str> = section.split(",").collect();

    let start_end_1: Vec<&str> = ranges[0].split("-").collect();
    let start_end_2: Vec<&str> = ranges[1].split("-").collect();

    let start_1: u32 = start_end_1[0].parse().unwrap();
    let start_2: u32 = start_end_2[0].parse().unwrap();

    if start_1 == start_2 {
        return true;
    }

    let end_1: u32 = start_end_1[1].parse().unwrap();
    let end_2: u32 = start_end_2[1].parse().unwrap();

    if end_1 == end_2 {
        return true;
    }

    if (start_1 < start_2 && end_1 > end_2) || (start_1 > start_2 && end_1 < end_2) {
        return true;
    }

    return false;
}

fn part2(sections: &str) {
    let mut overlap_pairs: u32 = 0;
    for section in sections.lines() {
        if any_overlaps(section) {
            overlap_pairs += 1;
        }
    }

    println!("part2: {}", overlap_pairs);
}

fn any_overlaps(section: &str) -> bool {
    let ranges: Vec<&str> = section.split(",").collect();

    let start_end_1: Vec<&str> = ranges[0].split("-").collect();
    let start_end_2: Vec<&str> = ranges[1].split("-").collect();

    let start_1: u32 = start_end_1[0].parse().unwrap();
    let start_2: u32 = start_end_2[0].parse().unwrap();
    let end_1: u32 = start_end_1[1].parse().unwrap();
    let end_2: u32 = start_end_2[1].parse().unwrap();

    let mut range1: HashSet<u32> = HashSet::new();
    let mut range2: HashSet<u32> = HashSet::new();

    for i in start_1..end_1 + 1 {
        range1.insert(i as u32);
    }

    for i in start_2..end_2 + 1 {
        range2.insert(i as u32);
    }

    return !range1.is_disjoint(&range2);
}
