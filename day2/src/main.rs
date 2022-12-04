use std::env;
use std::fs;
use std::collections::HashMap;

const DAY: &str = "day2";

#[derive(PartialEq)]
#[derive(Debug)]
enum RPSType {
    Rock,
    Paper,
    Scissors
}

struct RPS {
    id: RPSType,
    value: u32
}

const rock: RPS = RPS{
    id: RPSType::Rock,
    value: 1
};
const paper: RPS = RPS{
    id: RPSType::Paper,
    value: 2
};
const scissors: RPS = RPS{
    id: RPSType::Scissors,
    value: 3
};

#[derive(Debug)]
#[derive(PartialEq)]
enum Outcome{
    Win,
    Loss,
    Draw
}

fn main() {
    let file_path = format!("/{}/src/{}_test.txt", DAY, DAY);
    let file_path = format!("/{}/src/{}.txt", DAY, DAY);
    let strategy_guide = read_lines(&file_path);
    part1(&strategy_guide);
    part2(&strategy_guide);
}

fn read_lines(filename: &str) -> String{
    let root_dir = env::var("AOC_ROOT_DIR").expect("$AOC_ROOT_DIR is not set");
    let abs_filename = format!("{}{}", root_dir, filename);
    
    println!("Reading {}", abs_filename);
    let contents = fs::read_to_string(abs_filename)
       .expect("Something went wrong reading the file");

    return contents
}


fn part1(strategy_guide: &str) {
    let code_book = get_code_book_part1();
    let mut total_score: u32 = 0;

    for line in strategy_guide.lines(){
        let other = line.chars().nth(0).unwrap();
        let otherInstr = &code_book[&other];
        
        let mine = line.chars().nth(2).unwrap();
        let mineInstr = &code_book[&mine];
        
        total_score += get_score(mineInstr, otherInstr);
    }
    println!("part 1: {}", total_score);
}

fn get_code_book_part1() -> HashMap<char, RPS> {
    return HashMap::from([
        ('A', rock),
        ('B', paper),
        ('C', scissors),
        ('X', rock),
        ('Y', paper),
        ('Z', scissors),
    ]);
}

fn get_score(mine: &RPS, other: &RPS) -> u32{
    let mut outcome: Outcome = Outcome::Loss;

    if mine.id == other.id{
        outcome = Outcome::Draw;
    } else if mine.id == RPSType::Rock {
        if other.id == RPSType::Scissors{
            outcome = Outcome::Win;
        }
    } else if  mine.id == RPSType::Paper {
        if other.id == RPSType::Rock{
            outcome = Outcome::Win;
        }
    } else if mine.id == RPSType::Scissors {
        if other.id == RPSType::Paper{
            outcome = Outcome::Win;
        }
    }

    let mut score: u32 = match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    return score + mine.value;
}

// part 2

fn part2(strategy_guide: &str) {
    let code_book = get_code_book_part2();
    let code_outcomes = get_code_book_outcomes();
    let mut total_score: u32 = 0;

    for line in strategy_guide.lines(){
        let other = line.chars().nth(0).unwrap();
        let otherInstr = &code_book[&other];
        
        let outcome_code = line.chars().nth(2).unwrap();
        let outcome = &code_outcomes[&outcome_code];
        
        total_score += get_score_part2(outcome, otherInstr);
    }
    println!("part 2: {}", total_score);
}

fn get_code_book_part2() -> HashMap<char, RPS> {
    return HashMap::from([
        ('A', rock),
        ('B', paper),
        ('C', scissors),
    ]);
}

fn get_code_book_outcomes() -> HashMap<char, Outcome> {
    return HashMap::from([
        ('X', Outcome::Loss),
        ('Y', Outcome::Draw),
        ('Z', Outcome::Win),
    ]);
}

fn get_score_part2(outcome: &Outcome, other: &RPS) -> u32{
    let mut value = 0;

    if outcome == &Outcome::Draw{
        value = other.value;
    } else if other.id == RPSType::Rock {
        value = if outcome == &Outcome::Win { paper.value } else { scissors.value };
    } else if other.id == RPSType::Paper {
        value = if outcome == &Outcome::Win { scissors.value } else { rock.value };
    } else if other.id == RPSType::Scissors {
        value = if outcome == &Outcome::Win { rock.value } else { paper.value };
    }

    let mut score: u32 = match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    return score + value;
}