pub mod day_04 {

    use std::{error::Error, fs, num::ParseIntError};

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

        let prep = data
            .iter()
            .map(|l| {
                l.split(&[',', '-'])
                    .map(|v| v.parse::<i32>())
                    .collect::<Result<Vec<i32>, ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<i32>>, ParseIntError>>()?;

        //part1
        let p1 = prep
            .iter()
            .filter(|p| (p[0] >= p[2] && p[1] <= p[3]) || (p[0] <= p[2] && p[1] >= p[3]))
            .count();

        // part2
        let p2 = prep
            .iter()
            .filter(|p| p[1] >= p[2] && p[3] >= p[0])
            .count();

        Ok((p1.try_into()?, p2.try_into()?))
    }
}
