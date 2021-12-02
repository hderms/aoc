use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

enum Direction {
    Forward,
    Backward,
    Down,
    Up
}
impl FromStr for Direction {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "backward" => Ok(Direction::Backward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err("failed to parse enum".into())
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(file);
    let mut depth = 0;
    let mut horizontal = 0;
    for line in file.lines() {
        let line = line?;
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let direction = Direction::from_str(split[0])?;
        let magnitude: u64 = split[1].parse()?;
        match direction {
            Direction::Forward => {horizontal += magnitude}
            Direction::Backward => {horizontal -= magnitude}
            Direction::Down => {depth += magnitude}
            Direction::Up => {depth -= magnitude}
        }
    }
    println!("depth: {}, horizontal: {}, product: {}", depth, horizontal, depth * horizontal);
    Ok(())
}
