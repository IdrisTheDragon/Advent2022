pub mod day_02 {

    use std::{error::Error, fs, collections::HashMap};

    fn parse(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    
        let content = fs::read_to_string(&filename).map_err(|e| format!("Couldn't open {}: {}",filename,e))?;
    
        Ok(content.lines().into_iter().map(|l| l.to_string()).collect::<Vec<String>>())
    }
    ///
    /// Solve day 1 parts 1 and 2
    /// 
    /// ```
    /// # use day_02::day_02::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((15,12),x.unwrap());
    /// ```
    /// 
    pub fn solve(filename: &str) -> Result<(i32,i32), Box<dyn Error>> {
        let rounds = parse(filename)?;
        let elf = HashMap::from([
            ("A", "R"),
            ("B", "P"),
            ("C", "S"),
        ]);

        let player = HashMap::from([
            ("X", "R"),
            ("Y", "P"),
            ("Z", "S"),
        ]);

        let scoring = HashMap::from([
            ("R",1),
            ("P",2),
            ("S",3),
        ]);

        let wins = HashMap::from([
            ("R","S"),
            ("P","R"),
            ("S","P"),
        ]);

        let losses = HashMap::from([
            ("R","P"),
            ("P","S"),
            ("S","R"),
        ]);
    
        //part1
        let p1 = rounds.clone().into_iter().map( |l| -> i32 {
            
            let p = l.split(' ').collect::<Vec<&str>>();
            let e = elf.get(p[0]).expect(&format!("key: {:?} not in {:?}",p[0],elf.keys()));
            let pl = player.get(p[1]).expect(&format!("key: {:?} not in {:?}",p[1],player.keys())); 
            let s = scoring.get(pl).expect("A value").clone();
            
            if e == pl {
                // draw
                s + 3
            } else if wins.get(e).expect("A value") == pl {
                // e wins
                s
            } else {
                // p wins
                s+6
            }
            
            
        }).sum();

        // part2
        let p2 = rounds.into_iter().map( |l| -> i32 {
            
            let p = l.split(' ').collect::<Vec<&str>>();
            let e = elf.get(p[0]).expect(&format!("key: {:?} not in {:?}",p[0],elf.keys())).clone();
            let pl = p[1].clone(); 
            if pl == "X" {
                //e win
                scoring.get(wins.get(e).expect("A value")).expect("A value").clone()
            } else if pl == "Y" {
                //draw
                3 + scoring.get(e).expect("A value")
            } else {
                //pl win
                6 + scoring.get(losses.get(e).expect("A value")).expect("A value")
            }
            
            
        }).sum();
    

        Ok((p1,p2))
    }
    
}

