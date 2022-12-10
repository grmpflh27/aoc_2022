use std::env;
use std::fs;

const DAY: &str = "day10";

fn main() {
    let file_path = format!("/{}/src/{}_mini.txt", DAY, DAY);
    let file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let instrs = read_lines(&file_path);
    let X_recorder = part1(&instrs);
    part2(X_recorder);
}

fn read_lines(filename: &str) -> String {
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);

    println!("Reading {}", abs_filename);
    let contents = fs::read_to_string(abs_filename).expect("Something went wrong reading the file");

    return contents;
}

fn part1(instrs: &str) -> Vec<i32> {
    let mut X_recorder: Vec<i32> = Vec::new();
    let mut X = 1;
    let mut cycle = 1;

    for instr in instrs.lines() {
        //println!("{}: {}, {}", instr, cycle, X);

        if instr == "noop" {
            X_recorder.push(X);
            cycle += 1;
            continue;
        }
        let (_, val) = instr.split_once(' ').unwrap();
        let val: i32 = val.parse().unwrap();

        for i in 0..2 {
            X_recorder.push(X);
            cycle += 1;
        }
        X += val
    }
    X_recorder.push(X);

    let idxs: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let muls: Vec<i32> = vec![20, 60, 100, 140, 180, 220];

    let signals: Vec<i32> = idxs.iter().map(|idx| X_recorder[idx - 1]).collect();
    let sum_signal: i32 = signals
        .iter()
        .zip(muls.iter())
        .map(|(x, y)| x * y)
        .collect::<Vec<_>>()
        .iter()
        .sum();
    println!("part1: {}", sum_signal);

    return X_recorder;
}

fn part2(X_recorder: Vec<i32>) {
    let mut CRT: Vec<char> = Vec::new();

    let mut idx = 1;
    let mut row_cnt = 0;
    let row_size = 40;

    for X in X_recorder {
        let sprite_pos = X..X + 3;
        let mut cur_char = '.';

        if sprite_pos.contains(&idx) {
            cur_char = '#';
        }
        CRT.push(cur_char);

        if idx == row_size - 1 {
            row_cnt += 1;
            idx = -1;
        }
        idx += 1;
    }

    let CRT_str: String = CRT.iter().collect();
    println!("{:?}", &CRT_str[0..40]);
    println!("{:?}", &CRT_str[40..80]);
    println!("{:?}", &CRT_str[80..120]);
    println!("{:?}", &CRT_str[120..160]);
    println!("{:?}", &CRT_str[160..200]);
    println!("{:?}", &CRT_str[200..240]);
}
