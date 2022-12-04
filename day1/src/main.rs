use std::env;
use std::fs;

const DAY: &str = "day1";


fn main() {
    println!("TEST");
    let test_file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let yummy_food = read_lines(&test_file_path);
    let mut calories: Vec<u32> = parseCals(&yummy_food);
    part1(&calories);
    part2(calories);

    println!("FOR REAL");
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let yummy_food = read_lines(&file_path);
    
    let mut calories: Vec<u32> = parseCals(&yummy_food);
    part1(&calories);
    part2(calories);
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    
    println!("Reading {}", abs_filename);
    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}

fn parseCals(yummy_food: &str) -> Vec<u32> {
    let mut curCalCnt: u32 = 0;
    let mut calories: Vec<u32> = Vec::new();
    
    for line in yummy_food.lines(){
        if line.is_empty(){
            calories.push(curCalCnt);
            curCalCnt = 0;
            continue
        }
        curCalCnt += line.parse::<u32>().unwrap()
    }
    if curCalCnt != 0 {
        calories.push(curCalCnt);
    }
    return calories;
}

fn part1(calories: &Vec<u32>){
    println!("part1: {}", calories.iter().max().unwrap());
}


fn part2(mut calories: Vec<u32>){
    calories.sort_by(|a, b| a.cmp(b).reverse());
    println!("part 2: {:?}", calories[..3].iter().sum::<u32>());
}