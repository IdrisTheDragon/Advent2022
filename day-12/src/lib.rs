pub mod day_12 {
    #[derive(Debug, Clone)]
    struct Coord {
        y: usize,
        x: usize,
        neighbours: Vec<(usize, usize)>,
        distance: i32,
        height: char,
    }

    impl Coord {
        fn new(y: usize, x: usize, distance: i32, height: char) -> Self {
            let mut neighbours = Vec::new();

            neighbours.push((y, x + 1));
            neighbours.push((y + 1, x));
            if let Some(y1) = y.checked_sub(1) { neighbours.push((y1, x)) }
            if let Some(x1) = x.checked_sub(1) { neighbours.push((y, x1)) }

            Self {
                y,
                x,
                neighbours,
                distance,
                height,
            }
        }
    }

    use std::{collections::HashMap, error::Error, fs};

    fn parse(filename: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        Ok(content
            .lines()
            .map(|l| {
                let mut v = Vec::new();
                for c in l.to_string().chars() {
                    v.push(c)
                }
                v
            })
            .collect::<Vec<Vec<char>>>())
    }

    fn find_start(grid: &[Vec<char>]) -> Coord {
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'S' {
                    return Coord::new(y, x, 0, 'a');
                }
            }
        }
        Coord::new(0, 0, 0, 'a')
    }

    fn find_as(grid: &[Vec<char>]) -> Vec<Coord> {
        let mut aas = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'a' {
                    aas.push(Coord::new(y, x, 0, 'a'));
                }
            }
        }
        aas
    }

    fn get_char(y: usize, x: usize, grid: &[Vec<char>]) -> Option<char> {
        match grid.get(y) {
            Some(row) => row.get(x).copied(),
            None => None,
        }
    }

    ///
    /// Solve day_12 parts 1 and 2
    ///
    /// ```
    /// # use day_12::day_12::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((31,29),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(i32, i32), Box<dyn Error>> {
        let grid = parse(filename)?;

        let start = find_start(&grid);
        let mut finished = HashMap::new();
        let mut next = vec![start];

        let mut limit = 300000;
        while !finished.values().any(|v: &Coord| v.height == 'E') {
            let mut temp_next = Vec::new();

            for v in &next {
                finished.insert((v.y, v.x), v.clone());
                for (ny, nx) in &v.neighbours {
                    let f = finished.get_mut(&(*ny, *nx));
                    match f {
                        Some(f) => {
                            if f.distance < v.distance + 1 {
                                f.distance = v.distance + 1;
                            }
                        }
                        None => {
                            if let Some(c) = get_char(*ny, *nx, &grid) {
                                // if this is a valid neighbour node that's not been seen before add it to temp_next if it isn't their already.
                                let h = if c != 'E' { c } else { 'z' };
                                if (h as u8) <= (v.height as u8) + 1
                                    && !temp_next.iter().any(|t: &Coord| t.y == *ny && t.x == *nx)
                                {
                                    temp_next.push(Coord::new(*ny, *nx, v.distance + 1, c))
                                }
                            }
                        }
                    }
                }
            }

            next = temp_next;
            //println!("{:?} {:?}",finished.len(),finished.keys());
            //println!("{:?} {:?}\n",next.len(),next);
            limit -= 1;
            if limit == 0 {
                break;
            }
        }

        let end = finished
            .values()
            .find(|x| x.height == 'E')
            .ok_or("ohuh no end")?;
        //println!("{:?}",end.height);

        //part1
        let p1 = end.distance;

        let mut finished = HashMap::new();

        //Instead of single starting node with distance 0, start with all the a's
        let mut next = find_as(&grid);

        let mut limit = 300000;
        while !finished.values().any(|v: &Coord| v.height == 'E') {
            let mut temp_next = Vec::new();

            for v in &next {
                // put this node in finished and check all it's neighbours
                finished.insert((v.y, v.x), v.clone());
                for (ny, nx) in &v.neighbours {
                    let f = finished.get_mut(&(*ny, *nx));
                    match f {
                        Some(f) => {
                            // if we have seen this node before, see if it took us less distance this time.
                            if f.distance < v.distance + 1 {
                                f.distance = v.distance + 1;
                            }
                        }
                        None => {
                            if let Some(c) = get_char(*ny, *nx, &grid) {
                                // if this is a valid neighbour node that's not been seen before add it to temp_next if it isn't their already.
                                let h = if c != 'E' { c } else { 'z' };
                                if (h as u8) <= (v.height as u8) + 1
                                    && !temp_next.iter().any(|t: &Coord| t.y == *ny && t.x == *nx)
                                {
                                    temp_next.push(Coord::new(*ny, *nx, v.distance + 1, c))
                                }
                            }
                        }
                    }
                }
            }

            next = temp_next;
            //println!("{:?} {:?}",finished.len(),finished.keys());
            //println!("{:?} {:?}\n",next.len(),next);
            limit -= 1;
            if limit == 0 {
                break;
            }
        }

        let end = finished
            .values()
            .find(|x| x.height == 'E')
            .ok_or("ohuh no end")?;

        // part2
        let p2 = end.distance;

        Ok((p1, p2))
    }
}
