use std::error::Error;

use day_01::day_01;
use day_02::day_02;
//{{next day import}}

fn main() -> Result<(), Box<dyn Error>> {
    let x = day_01::solve("day-01/input.txt")?;
    println!("Day 1 Part1: {}", x.0);
    println!("Day 1 Part2: {}", x.1);

    let x = day_02::solve("day-02/input.txt")?;
    println!("Day 2 Part1: {}", x.0);
    println!("Day 2 Part2: {}", x.1);
    
    //{{next day}}
    Ok(())
}
