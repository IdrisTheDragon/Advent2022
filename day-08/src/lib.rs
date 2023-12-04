pub mod day_08 {

    use std::{error::Error, fs};

    fn parse(filename: &str) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
        let content = fs::read_to_string(&filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        let x = content
            .lines()
            .map(|l| {
                l.chars()
                    .map(|x| x.to_digit(10).ok_or("A problem"))
                    .collect()
            })
            .collect::<Result<Vec<Vec<u32>>, &str>>()?;
        Ok(x)
    }

    ///
    /// Solve day_08 parts 1 and 2
    ///
    /// ```
    /// # use day_08::day_08::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((21,8),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(usize, usize), Box<dyn Error>> {
        let data = parse(filename)?;

        let mut count = 0;
        let mut best_view = 0;

        // edges
        count += (data.len() - 2) * 2;
        count += data[0].len() * 2;

        for row in 1..data.len() - 1 {
            for col in 1..data[0].len() - 1 {
                let r = data.get(row).ok_or("missing r")?;
                let h = r.get(col).ok_or("missing v")?;

                //p1
                if
                //row left
                r[0..col].iter().max().ok_or("no max")? < h
                ||
                //row right
                r[col+1..r.len()].iter().max().ok_or("no max")? < h
                ||
                //col above
                data[0..row].iter()
                    .map(|v| v[col])
                    .max().ok_or("no max")? < *h
                ||
                // col below
                data[row+1..data.len()].iter()
                    .map(|v| v[col])
                    .max().ok_or("no max")? < *h
                {
                    count += 1;
                }

                //p2
                //println!("{:?}", &data[row+1..data.len()]);
                let mut last = true;
                let scores = vec![
                    //row left
                    r[0..col]
                        .iter()
                        .rev()
                        .take_while(|x| {
                            last = *x < h;
                            last
                        })
                        .count()
                        + if last { 0 } else { 1 },
                    //row right
                    r[col + 1..r.len()]
                        .iter()
                        .take_while(|x| {
                            last = *x < h;
                            last
                        })
                        .count()
                        + if last { 0 } else { 1 },
                    //col above
                    data[0..row]
                        .iter()
                        .map(|v| v[col])
                        .rev()
                        .take_while(|x| {
                            last = x < h;
                            last
                        })
                        .count()
                        + if last { 0 } else { 1 },
                    //col below
                    data[row + 1..data.len()]
                        .iter()
                        .map(|v| v[col])
                        .take_while(|x| {
                            last = x < h;
                            last
                        })
                        .count()
                        + if last { 0 } else { 1 },
                ];
                //println!("{} {} {:?}",row,col,scores);
                let score = scores.iter().product();
                if score > best_view {
                    best_view = score;
                }
            }
        }

        //part1
        let p1 = count;

        // part2
        let p2 = best_view;

        Ok((p1, p2))
    }
}
