
use std::error::Error;

use day_01::day_01;
fn main() -> Result<(), Box<dyn Error>> {
    let x = day_01::solve("day_01/input.txt")?;
    println!("Day 1 Part1: {}",x.0);
    println!("Day 1 Part2: {}",x.1);
    
    Ok(())
}
