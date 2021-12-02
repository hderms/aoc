use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let buffered = BufReader::new(file);
    let mut previous_measurement: Option<u64> = None;
    let mut count = 0;
    for line in buffered.lines() {
        let line = line?;
        let new_measurement = line.parse()?;
        match previous_measurement {
            Some(previous) if new_measurement > previous => {
                count += 1
            }
            _ => (),
        }
        previous_measurement = Some(new_measurement);

    }
    println!("count is: {}", count);

    println!("Hello, world!");
    Ok(())
}
