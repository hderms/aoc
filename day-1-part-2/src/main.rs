use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let file = BufReader::new(file);
    let parsed: Vec<u64> = file.lines().map(|line| line.unwrap().parse().unwrap()).collect();
    let mut count = 0;
    let mut previous_value: Option<u64> = None;
    for window in parsed.windows(3) {
        let mut sum = 0;
        for element in window {
            sum += element
        }
        match previous_value {
            Some(previous) if previous < sum => {
                count += 1

            }
            _ => {


            }
        }
        previous_value = Some(sum);

    }

    println!("{}", count);
    Ok(())
}
