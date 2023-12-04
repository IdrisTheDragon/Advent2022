pub mod day_06 {

    use std::{collections::HashSet, error::Error, fs};

    fn parse(filename: &str) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(&filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        Ok(content.trim().to_string())
    }

    ///
    /// Solve day_06 parts 1 and 2
    ///
    /// ```
    /// # use day_06::day_06::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((7,19),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(usize, usize), Box<dyn Error>> {
        let data = parse(filename)?.chars().collect::<Vec<char>>();

        let (idx, _) = data
            .clone()
            .windows(4)
            .enumerate()
            .find(|(_, c)| {
                let mut set = HashSet::new();
                for x in *c {
                    set.insert(x);
                }
                set.len() == 4
            })
            .expect("a result");

        //part1
        let p1 = idx + 4;

        let (idx_p2, _) = data
            .clone()
            .windows(14)
            .enumerate()
            .find(|(_, c)| {
                let mut set = HashSet::new();
                for x in *c {
                    set.insert(x);
                }
                set.len() == 14
            })
            .expect("a result");

        // part2
        let p2 = idx_p2 + 14;

        Ok((p1, p2))
    }
}
