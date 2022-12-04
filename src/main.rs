use std::error::Error;

use std::time::Instant;

use day_01::day_01;
use day_02::day_02;
use day_03::day_03;
use day_04::day_04;
//{{next day import}}

fn main() -> Result<(), Box<dyn Error>> {
    let now_01 = Instant::now();
    let x = day_01::solve("day-01/input.txt")?;
    println!("=== Day 01 ===");
    println!("Part 1: {}", x.0);
    println!("Part 2: {}", x.1);
    println!("Elapsed: {:.2?}", now_01.elapsed());

    let now = Instant::now();
    let x = day_02::solve("day-02/input.txt")?;
    println!("=== Day 02 ===");
    println!("Part 1: {}", x.0);
    println!("Part 2: {}", x.1);
    println!("Elapsed: {:.2?}", now.elapsed());
    
    let now = Instant::now();
    let x = day_03::solve("day-03/input.txt")?;
    println!("=== Day 03 ===");
    println!("Part 1: {}", x.0);
    println!("Part 2: {}", x.1);
    println!("Elapsed: {:.2?}", now.elapsed());
    
    let now = Instant::now();
    let x = day_04::solve("day-04/input.txt")?;
    println!("=== Day 04 ===");
    println!("Part 1: {}", x.0);
    println!("Part 2: {}", x.1);
    println!("Elapsed: {:.2?}", now.elapsed());
    
    //{{next day}}

    println!("=== Finished ===");
    println!("Total Elapsed: {:.2?}", now_01.elapsed());
    Ok(())
}
