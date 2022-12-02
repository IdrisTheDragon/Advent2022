pub mod {{project-name | snake_case}} {

    use std::{error::Error, fs};

    fn parse(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    
        let content = fs::read_to_string(&filename).map_err(|e| format!("Couldn't open {}: {}",filename,e))?;
    
        Ok(content.lines().into_iter().map(|l| l.to_string()).collect::<Vec<String>>())
    }


    ///
    /// Solve {{project-name | snake_case}} parts 1 and 2
    /// 
    /// ```
    /// # use {{project-name | snake_case}}::{{project-name | snake_case}}::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((0,0),x.unwrap());
    /// ```
    /// 
    pub fn solve(filename: &str) -> Result<(i32,i32), Box<dyn Error>> {
        let data = parse(filename)?;
    
        //part1
        let p1 = 0;

        // part2
        let p2 = 0;
    
        Ok((p1,p2))
    }
    
}

