pub mod day_09 {

    use std::{collections::HashSet, error::Error, fs};

    fn parse(filename: &str) -> Result<Vec<char>, Box<dyn Error>> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        Ok(content
            .lines()
            .map(|l| {
                let mut x = l.split(' ');
                (
                    x.next()
                        .expect("something")
                        .parse::<char>()
                        .expect("a char"),
                    x.next()
                        .expect("something")
                        .parse::<usize>()
                        .expect("a number"),
                )
            })
            .flat_map(|(x, y)| vec![x; y])
            .collect::<Vec<char>>())
    }

    // if you really want to output it
    #[allow(dead_code)]
    fn output(
        historyp1: &HashSet<(i32, i32)>,
        historyp2: &HashSet<(i32, i32)>,
        points: &Vec<(i32, i32)>,
    ) {
        let mut all = historyp1.clone();
        all.extend(historyp2);
        all.extend(points);

        let miny = all.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap_or(&(-1, -1));
        let minx = all.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap_or(&(-1, -1));
        let maxy = all.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap_or(&(2, 2));
        let maxx = all.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap_or(&(2, 2));

        let mut grid = vec![
            vec!['.'; (maxy.1 + miny.1.abs() + 3) as usize];
            (maxx.0 + minx.0.abs() + 3) as usize
        ];

        historyp1.iter().for_each(|(x, y)| {
            grid[(*x + minx.0.abs()) as usize][(*y + miny.1.abs()) as usize] = '#'
        });
        historyp2.iter().for_each(|(x, y)| {
            grid[(*x + minx.0.abs()) as usize][(*y + miny.1.abs()) as usize] = '@'
        });

        let s = (0, 0);
        grid[(s.0 + minx.0.abs()) as usize][(s.1 + miny.1.abs()) as usize] = 's';
        points.iter().for_each(|tail| {
            grid[(tail.0 + minx.0.abs()) as usize][(tail.1 + miny.1.abs()) as usize] = 'H'
        });

        grid.reverse();
        for r in grid {
            for v in r {
                print!("{}", v);
            }
            println!();
        }
        println!();
    }

    ///
    /// Solve day_09 parts 1 and 2
    ///
    /// ```
    /// # use day_09::day_09::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((13,1),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(usize, usize), Box<dyn Error>> {
        let data = parse(filename)?;
        let mut points = vec![(0, 0); 10];
        let mut visitedp1 = HashSet::new();
        let mut visitedp2 = HashSet::new();

        data.iter().for_each(|c| {
            //output(&visitedp1,&visitedp2,&points);
            match c {
                'R' => points[0].1 += 1,
                'L' => points[0].1 -= 1,
                'U' => points[0].0 += 1,
                'D' => points[0].0 -= 1,
                _ => todo!(),
            }

            for idx in 1..points.len() {
                if ((points[idx - 1].0 - points[idx].0) as i32).abs() < 2
                    && ((points[idx - 1].1 - points[idx].1) as i32).abs() < 2
                {
                    //nothing todo
                } else if points[idx - 1].0 == points[idx].0 {
                    points[idx].1 += if points[idx - 1].1 < points[idx].1 {
                        -1
                    } else {
                        1
                    };
                } else if points[idx].1 == points[idx - 1].1 {
                    points[idx].0 += if points[idx - 1].0 < points[idx].0 {
                        -1
                    } else {
                        1
                    };
                } else {
                    points[idx].0 += if points[idx - 1].0 < points[idx].0 {
                        -1
                    } else {
                        1
                    };
                    points[idx].1 += if points[idx - 1].1 < points[idx].1 {
                        -1
                    } else {
                        1
                    };
                }
                if idx == 1 {
                    visitedp1.insert(points[1]);
                } else if idx == 9 {
                    visitedp2.insert(points[9]);
                }
            }
        });

        //part1
        let p1 = visitedp1.len();

        // part2
        let p2 = visitedp2.len();

        Ok((p1, p2))
    }
}
