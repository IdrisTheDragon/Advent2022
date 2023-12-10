pub mod day_03 {

    use std::{collections::HashSet, error::Error, fs};

    fn parse(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        Ok(content
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>())
    }

    ///
    /// Solve day_03 parts 1 and 2
    ///
    /// ```
    /// # use day_03::day_03::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((157,70),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(i32, i32), Box<dyn Error>> {
        let data = parse(filename)?;
        let mut both = Vec::new();
        let mut badges = Vec::new();
        let mut sets = Vec::new();
        for (idy, rucksack) in data.iter().enumerate() {
            let mut half = HashSet::new();

            for (idx, c) in rucksack.chars().enumerate() {
                if idx < &rucksack.len() / 2 {
                    half.insert(c);
                } else if half.contains(&c) {
                    both.push(c);
                    break;
                }
            }
            let full = HashSet::<char>::from_iter(rucksack.chars());
            sets.push(full);
            if idy % 3 == 2 {
                assert!(sets.len() >= 3);
                let s = sets[idy]
                    .intersection(&sets[idy - 1])
                    .copied()
                    .collect::<HashSet<char>>();
                let t = s
                    .intersection(&sets[idy - 2])
                    .copied()
                    .collect::<Vec<char>>();
                assert!(t.len() == 1);
                badges.push(t[0]);
            }
        }
        //part1
        let p1 = both
            .into_iter()
            .map(|c| {
                if c.is_ascii_lowercase() {
                    c as i32 - 96
                } else {
                    c as i32 - 64 + 26
                }
            })
            .sum();

        // part2
        let p2 = badges
            .into_iter()
            .map(|c| {
                if c.is_ascii_lowercase() {
                    c as i32 - 96
                } else {
                    c as i32 - 64 + 26
                }
            })
            .sum();

        Ok((p1, p2))
    }
}
