use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::String;
use std::str::FromStr;

#[derive(Debug)]
struct Range(i32, i32);

fn str_to_nested(s: &String) -> i32
{
    let sv: Vec<&str> = s.split(",").collect();
    assert_eq!(sv.len(), 2);
    return ranges_nested(str_to_range(&sv[0].to_string()), str_to_range(&sv[1].to_string()));
}

fn str_to_overlap(s: &String) -> i32
{
    let sv: Vec<&str> = s.split(",").collect();
    assert_eq!(sv.len(), 2);
    return ranges_overlap(str_to_range(&sv[0].to_string()), str_to_range(&sv[1].to_string()));
}

fn str_to_range(s: &String) -> Range
{
    let sv: Vec<&str> = s.split("-").collect();
    assert_eq!(sv.len(), 2);

    let i0 = match i32::from_str(sv[0])
    {
        Ok(i) => i,
        _ =>  panic!("Parsing error!"),
    };

    let i1 = match i32::from_str(sv[1])
    {
        Ok(i) => i,
        _ =>  panic!("Parsing error!"),
    };

    Range(i0,i1)
}

fn ranges_nested(r0: Range, r1: Range) -> i32 
{
    if (r0.0 <= r1.0 && r0.1 >= r1.1) || (r1.0 <= r0.0 && r1.1 >= r0.1)
    {
        1
    }
    else
    {
        0
    }
}

fn ranges_overlap(r0: Range, r1: Range) -> i32 
{
    let l0 = r0.1 - r0.0;
    let l1 = r1.1 - r1.0;
    if (r0.0 <= r1.0 && r0.0 + l0 >= r1.0) || (r1.0 <= r0.0 && r1.0 + l1 >= r0.0)
    {
        1
    }
    else
    {
        0
    }
}

fn main() {
    if let Ok(v) = read_lines("input.txt") {
        let x: i32 = v.iter().map(|s| str_to_nested(&s)).sum();
        println!("Number of nested: {}", x);
        let y: i32 = v.iter().map(|s| str_to_overlap(&s)).sum();
        println!("Number of overlap: {}", y);
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
