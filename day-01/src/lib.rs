pub mod day_01 {

    use std::{error::Error, fs};

    fn parse(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
        let content = fs::read_to_string(&filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        let mut elves = Vec::new();
        let mut elf = 0;

        for l in content.lines() {
            if l.is_empty() {
                elves.push(elf);
                elf = 0;
            } else {
                elf += l
                    .parse::<i32>()
                    .map_err(|_| format!("Failed to parse: '{}'", l))?;
            }
        }
        elves.push(elf);

        Ok(elves)
    }
    ///
    /// Solve day 1 parts 1 and 2
    ///
    /// ```
    /// # use day_01::day_01::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((24000,45000),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(i32, i32), Box<dyn Error>> {
        let mut elves = parse(filename)?;

        //part1
        //let p1 = elves.into_iter().reduce(|x,y| x.max(y)).ok_or("No number to add")?;

        //part2
        elves.sort();
        elves.reverse();
        let p1 = elves[0];
        elves.truncate(3);
        let p2 = elves.into_iter().sum();

        Ok((p1, p2))
    }
}
