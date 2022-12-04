pub mod day_04 {

    use std::{error::Error, fs};

    fn parse(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let content = fs::read_to_string(&filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        Ok(content
            .lines()
            .into_iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>())
    }

    ///
    /// Solve day_04 parts 1 and 2
    ///
    /// ```
    /// # use day_04::day_04::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((2,4),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(i32, i32), Box<dyn Error>> {
        let data = parse(filename)?;

        let prep = data.iter().map(|l| {
            l.split(',')
                .map(|m| {
                    m.split('-')
                        .map(|v| v.parse::<i32>().expect("A value"))
                })
                .fold(Vec::<i32>::new(),|mut acc, m| {
                    acc.extend(m);
                    acc
                })
        });

        //part1
        let p1 = prep
            .clone()
            .map(|p| (p[0] >= p[2] && p[1] <= p[3]) || (p[0] <= p[2] && p[1] >= p[3]))
            .fold(0, |acc, v| if v { acc + 1 } else { acc });

        // part2
        let p2 = prep
            .map(|p| p[1] >= p[2] && p[3] >= p[0])
            .fold(0, |acc, v| if v { acc + 1 } else { acc });

        Ok((p1, p2))
    }
}
