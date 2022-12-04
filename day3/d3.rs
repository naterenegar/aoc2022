use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::String;


fn find_double_item(s: &String) -> char
{
    let l = s.len();
    if l % 2 != 0 {
        panic!("Bad string in input! Odd number of items in rucksack!")
    }
    let c1_str = &s[0..(l/2)];
    let c2_str = &s[(l/2)..];
    for c in c1_str.chars() 
    {
        if c2_str.contains(c) {
            return c
        }
    }

    ' '
}

fn find_badge(s: &[String]) -> char
{
    let l = s.len();
    if l % 3 != 0 {
        panic!("Not given a group of 3 elves!")
    }
    for c in s[0].chars() 
    {
        if s[1].contains(c) && s[2].contains(c) {
            return c
        }
    }

    ' '
}

fn char_to_priority(c: char) -> u32
{
    let x = u32::from(c);
    if x >= 65 && x <= 90
    {
        return x - 65 + 27; 
    } 
    else if x >= 97 && x <= 122
    {
        return x - 96; 
    }
    else
    {
        return 0;
    }
}

fn main() {
    if let Ok(v) = read_lines("input.txt") {
/*        for s in v.iter()
        {
            println!("{}", find_double_item(&s));
        }
        */
        let tot_prio: u32 = v.iter().map(|s| char_to_priority(find_double_item(&s))).sum();
        println!("Total priority: {}", tot_prio); 
        let badge_prio: u32 = v.chunks(3).map(|s| char_to_priority(find_badge(&s))).sum();
        println!("Badge priority: {}", badge_prio); 
    }
    else
    {
        println!("input.txt not found");
    }
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let v = io::BufReader::new(file).lines().collect::<Result<Vec<_>, _>>()?;
    Ok(v)
}
