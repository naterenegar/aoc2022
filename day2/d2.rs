use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::String;

enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn str_to_val(s: String) -> i32
{
    let mut i = s.split_whitespace();
    let h0 = match i.next() {
        Some("A") => Hand::Rock,
        Some("B") => Hand::Paper,
        Some("C") => Hand::Scissors,
        _ => panic!("Bad string!"), 
    };
    let h1 = match i.next() {
        Some("X") => Hand::Rock,
        Some("Y") => Hand::Paper,
        Some("Z") => Hand::Scissors,
        _ => panic!("Bad string!"),
    };
    eval_game(h0, h1)
}

fn str_to_val_p2(s: String) -> i32
{
    let mut i = s.split_whitespace();
    let h0 = match i.next() {
        Some("A") => Hand::Rock,
        Some("B") => Hand::Paper,
        Some("C") => Hand::Scissors,
        _ => panic!("Bad string!"), 
    };
    let h1 = match i.next() {
        Some("X") => match h0 {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        },
        Some("Y") => match h0 {
            Hand::Rock => Hand::Rock,
            Hand::Paper => Hand::Paper,
            Hand::Scissors => Hand::Scissors,
        },
        Some("Z") => match h0 {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        },
        _ => panic!("Bad string!"),
    };
    eval_game(h0, h1)
}

fn eval_game(their_hand: Hand, my_hand: Hand) -> i32 {
    let i = match their_hand {
        Hand::Rock => {
            match my_hand {
                Hand::Rock => 3, // tie
                Hand::Paper => 6, // win
                Hand::Scissors => 0, // lose
            }
        },
        Hand::Paper => {
            match my_hand {
                Hand::Rock => 0, // lose
                Hand::Paper => 3, // tie
                Hand::Scissors => 6, // win
            }
        },
        Hand::Scissors => {
            match my_hand {
                Hand::Rock => 6, // win
                Hand::Paper => 0, // lose
                Hand::Scissors => 3, // tie
            }
        }
    };
    i + hand_value(my_hand)
}

fn hand_value(hand: Hand) -> i32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn main() {
    let mut x = 0;
    let mut y = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                x = x + str_to_val(ip.clone());
                y = y + str_to_val_p2(ip);
            }
        }
    }
    println!("Total points: {}", x);
    println!("Total points (p2): {}", y);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
