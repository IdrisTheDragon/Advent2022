pub mod day_11 {

    use itertools::Itertools;
    use std::collections::VecDeque;
    use std::{error::Error, fs};

    #[derive(Debug)]
    struct Monkey {
        items: VecDeque<i64>,
        op: char,
        op_v: Box<str>,
        test_v: i64,
        test_true: usize,
        test_false: usize,
        inspections: i64,
    }

    impl Monkey {
        fn new(input: &str) -> Monkey {
            let mut data = input.lines();
            data.next(); // Monkey 0:
                         //   Starting items: 79, 98
            let items = data
                .next()
                .expect("start")
                .split([':', ','])
                .skip(1)
                .map(|v| v.trim().parse::<i64>().expect("success"))
                .collect::<VecDeque<i64>>();
            //   Operation: new = old * 19
            let mut op_line: Vec<&str> = data.next().expect("op").split(' ').collect();

            println!("{:?}", op_line[7]);

            Monkey {
                items,
                op: op_line[6].chars().next().expect("op2"),
                op_v: op_line[7].into(),
                test_v: data
                    .next()
                    .expect("t")
                    .split(" ")
                    .nth(5)
                    .expect("t1")
                    .parse::<i64>()
                    .expect("t2"),
                test_true: data
                    .next()
                    .expect("tt")
                    .split(" ")
                    .nth(9)
                    .expect("ttt1")
                    .parse::<usize>()
                    .expect("tt2"),
                test_false: data
                    .next()
                    .expect("tf")
                    .split(" ")
                    .nth(9)
                    .expect("tf1")
                    .parse::<usize>()
                    .expect("tf2"),
                inspections: 0,
            }
        }
        fn inspect(&mut self) -> Option<(usize, i64)> {
            // inspect
            let mut item = self.items.pop_front()?;
            self.inspections += 1;
            let op_v = match self.op_v.parse::<i64>() {
                Ok(v) => v,
                Err(_) => item.clone(),
            };
            match self.op {
                '+' => item += op_v,
                '*' => item *= op_v,
                _ => {}
            }

            item /= 3;

            let throw = if item % self.test_v == 0 {
                self.test_true
            } else {
                self.test_false
            };

            Some((throw, item))
        }
    }

    fn parse(filename: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        Ok(content
            .split("\n\n")
            .map(Monkey::new)
            .collect::<Vec<Monkey>>())
    }

    ///
    /// Solve day_11 parts 1 and 2
    ///
    /// ```
    /// # use day_11::day_11::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((10605,0),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(i64, i64), Box<dyn Error>> {
        let mut data = parse(filename)?;

        //println!("{:?}\n", data);

        let mut round = 1;
        while round <= 20 {
            round += 1;
            let mut mcount = 0;
            while mcount < data.len() {
                while let Some((throw, item)) = data[mcount].inspect() {
                    data[throw].items.push_back(item);
                }
                mcount += 1;
            }
            //println!("{:?}\n", data);
        }

        //part1
        let p1 = data
            .into_iter()
            .map(|m| m.inspections)
            .sorted()
            .rev()
            .take(2)
            .map(|c| {
                println! {"{}",c};
                c
            })
            .product();

        // part2
        let p2 = 0;

        Ok((p1, p2))
    }
}
