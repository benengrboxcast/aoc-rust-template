use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    use std::time::Instant;

    let mut start = Instant::now();
    let mut result;
    {
        result = part1();
    }
    
    let mut elapsed = start.elapsed();
    match result {
        Ok(s) => println!("Part 1 {} in {:.4?}\n", s, elapsed),
        Err(err) => panic!("Failure {}\n", err)
    }

    start = Instant::now();
    result = part2();
    elapsed = start.elapsed();
    match result {
        Ok(s) => println!("part 2 {} in {:.4?}\n", s, elapsed),
        Err(err) => panic!("Failure {}\n", err)
    }
}

fn part1() -> Result<u32, Box<dyn std::error::Error>> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);
    let mut result: u32 = 0;

    for line in reader.lines() {

        match line {
            Ok(s) => {
                result += s.len() as u32;
            }
            Err(err) => panic!("{}", err)
        }
    }
    Ok(result)
}

fn part2() -> Result<u32, Box<dyn std::error::Error>> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);
    let mut result: u32 = 0;

    for line in reader.lines() {

        match line {
            Ok(s) => {
                result += s.len() as u32;
            }
            Err(err) => panic!("{}", err)
        }
    }
    Ok(result)
}