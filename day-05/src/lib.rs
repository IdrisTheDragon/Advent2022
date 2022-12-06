pub mod day_05 {

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
    /// Solve day_05 parts 1 and 2
    ///
    /// ```
    /// # use day_05::day_05::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!(("CMZ".to_string(),"MCD".to_string()),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(String, String), Box<dyn Error>> {
        let data = parse(filename)?;

        let stackcount = (data[0].len() +1) / 4;
        let mut stacks = Vec::new();
        for _ in 0..stackcount {
            stacks.push(Vec::new());
        }

        // parse the stacks
        data.iter()
            .take_while(|a| !a.starts_with(" 1"))
            .for_each(|f| {
                stacks.iter_mut().enumerate().for_each(|(idx, stack)| {
                    let c = f.chars().nth(idx * 4 + 1).unwrap(); //todo safely
                    if c.is_alphabetic() {
                        stack.push(c);
                    }
                    
                });
            });

        // parse instructions,
        let instructions = data.iter()
            // skip until after the empty line
            .skip_while(|a| !a.is_empty())
            .skip(1)
            // parse the lines to usable data
            .map(|l| {
                let l1: Vec<&str> = l.split(' ').collect();
                (
                    l1[1].parse::<usize>().unwrap(),
                    l1[3].parse::<usize>().unwrap(),
                    l1[5].parse::<usize>().unwrap(),
                )
            });
        
        // save copy for part 2
        let mut p2_stacks = stacks.clone();

        //run the instructions p1 way
        instructions.clone().for_each(|(num, from, to)| {
                let from_stack= stacks.get_mut(from-1).expect("a stack");

                let mut temp = Vec::new();
                
                from_stack.drain(0..num).for_each(|a| temp.push(a));
                
                
                let stack = stacks.get_mut(to-1).expect("a stack");

                stack.reverse();
                stack.extend(temp);
                stack.reverse();
            });

        //part1
        let p1 = stacks.iter().map(|stack| stack.first().expect("a value in the stack")).collect::<String>();

        //run the instructions p2 way
        instructions.for_each(|(num, from, to)| {
            let from_stack= p2_stacks.get_mut(from-1).expect("a stack");

            let mut temp = Vec::new();
            
            from_stack.drain(0..num).for_each(|a| temp.push(a));

            temp.reverse();
            
            
            let stack = p2_stacks.get_mut(to-1).expect("a stack");

            stack.reverse();
            stack.extend(temp);
            stack.reverse();
        });

        // part2
        let p2 = p2_stacks.iter().map(|stack| stack.first().expect("a value in the stack")).collect::<String>();

        Ok((p1, p2))
    }
}
