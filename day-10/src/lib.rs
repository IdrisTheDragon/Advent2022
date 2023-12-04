pub mod day_10 {

    use std::{error::Error, fs};

    fn parse(filename: &str) -> Result<Vec<(String, i32)>, Box<dyn Error>> {
        let content = fs::read_to_string(&filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        Ok(content
            .lines()
            .into_iter()
            .map(|l| {
                let mut t = l.split(" ");
                (
                    t.next().expect("a str").to_string(),
                    t.next().unwrap_or("0").parse::<i32>().expect("a num"),
                )
            })
            .collect::<Vec<(String, i32)>>())
    }

    fn run_cycle(screen: &mut Vec<char>, strengths: &mut Vec<i32>, cycle: &mut i32, x: &mut i32) {
        *cycle += 1;
        strengths.push(*x * *cycle);

        screen.push(
            if *x - 1 <= (*cycle - 1) % 40 && (*cycle - 1) % 40 <= *x + 1 {
                '#'
            } else {
                '.'
            },
        );
        // if -1 < *cycle && *cycle <15 {
        //     for x in screen {
        //         print!("{}",x);
        //     }
        //     println!(" {} {}",cycle,x);
        // }
    }

    ///
    /// Solve day_10 parts 1 and 2
    ///
    ///
    pub fn solve(filename: &str) -> Result<(i32, String), Box<dyn Error>> {
        let data = parse(filename)?;

        let mut cycle = 0;
        let mut x = 1;
        let mut strengths = Vec::new();
        let mut screen = Vec::new();

        for (i, v) in data {
            match i.as_str() {
                "addx" => {
                    run_cycle(&mut screen, &mut strengths, &mut cycle, &mut x);
                    run_cycle(&mut screen, &mut strengths, &mut cycle, &mut x);
                    x += v;
                }
                "noop" => {
                    run_cycle(&mut screen, &mut strengths, &mut cycle, &mut x);
                }
                _ => {}
            }
        }

        let values = [
            strengths[20 - 1],
            strengths[60 - 1],
            strengths[100 - 1],
            strengths[140 - 1],
            strengths[180 - 1],
            strengths[220 - 1],
        ];
        //println!("{:?}", values);
        //part1
        let p1 = values.iter().sum();

        let mut out = "".to_string();
        for x in 0..6 {
            for y in 0..40 {
                out += &screen[x * 40 + y].to_string();
            }
            out += "\n";
        }

        // part2
        let p2 = out;

        Ok((p1, p2))
    }
}
