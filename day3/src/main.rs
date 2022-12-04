use counter::Counter;
use std::env;
use std::fs;

const DAY: &str = "day3";

fn main() {
    let file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let backpacks = read_lines(&file_path);
    part1(&backpacks);
    part2(&backpacks);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    println!("Reading {}", abs_filename);
    let contents = fs::read_to_string(abs_filename).expect("Something went wrong reading the file");

    return contents;
}

fn part1(backpacks: &str) {
    let mut priority: u32 = 0;
    for backpack in backpacks.lines() {
        let (comp1, comp2) = backpack.split_at(backpack.len() / 2);
        let cnt1 = comp1.chars().collect::<Counter<_>>();
        let cnt2 = comp2.chars().collect::<Counter<_>>();

        let intersection = cnt1 & cnt2;
        let duplicate = intersection.most_common()[0].0;

        priority += get_prio(duplicate);
    }

    println!("part1: {}", priority);
}

fn get_prio(duplicate: char) -> u32 {
    let mut prio = 0;
    if duplicate.is_lowercase() {
        prio = duplicate as u32 - 96;
    } else {
        prio = duplicate as u32 - 38;
    }
    return prio;
}

fn part2(backpacks: &str) {
    let mut priority: u32 = 0;
    let lines: Vec<&str> = backpacks.split('\n').collect();

    let mut cnt = 0;
    while cnt < lines.len() {
        let cnt1 = lines[cnt].chars().collect::<Counter<_>>();
        let cnt2 = lines[cnt + 1].chars().collect::<Counter<_>>();
        let cnt3 = lines[cnt + 2].chars().collect::<Counter<_>>();

        let intersection = cnt1 & cnt2 & cnt3;
        let duplicate = intersection.most_common()[0].0;

        priority += get_prio(duplicate);

        cnt += 3
    }
    println!("part2: {}", priority);
}
