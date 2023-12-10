pub mod day_07 {

    use std::{
        collections::{HashMap, HashSet},
        error::Error,
        fs,
    };

    fn parse(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Couldn't open {}: {}", filename, e))?;

        Ok(content
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>())
    }

    ///
    /// Solve day_07 parts 1 and 2
    ///
    /// ```
    /// # use day_07::day_07::solve;
    /// let x = solve("example.txt");
    /// assert!(x.is_ok());
    /// assert_eq!((95437,24933642),x.unwrap());
    /// ```
    ///
    pub fn solve(filename: &str) -> Result<(u64, u64), Box<dyn Error>> {
        let data = parse(filename)?;

        let mut dir = "".to_string();
        let mut files = HashMap::new();
        let mut dirs = HashSet::new();

        data.iter().for_each(|l| {
            if l.starts_with("$ ") {
                let mut l1 = l.split(' ');
                match l1.nth(1) {
                    Some(x) if x == "cd" => match l1.next() {
                        Some(y) if y == ".." => {
                            let mut last = 't';
                            dir = dir
                                .trim_end_matches(|c| {
                                    if last == '/' {
                                        false
                                    } else {
                                        last = c;
                                        true
                                    }
                                })
                                .to_string();
                            //dirs.insert(dir.clone());
                        }
                        Some(y) if y == "/" => {
                            dir = "".to_string();
                            dirs.insert("/".to_string());
                        }
                        Some(y) => {
                            dir = dir.clone() + "/" + y;
                            dirs.insert(dir.clone());
                        }
                        None => panic!(),
                    },
                    //ignore ls
                    Some(_) => (),
                    None => panic!(),
                }
            } else if l.starts_with("dir") {
                // ignore dirs
                
            } else {
                let l1 = l.split(' ').collect::<Vec<&str>>();
                files.insert(
                    dir.clone() + "/" + l1.get(1).expect("a file name"),
                    l1.first().expect("a value").parse::<u64>().expect("success"),
                );
            }
        });

        //println!("{:?}",files);
        //println!("{:?}",dirs);

        let dir_sizes = dirs.iter().map(|dir| {
            let dir1 = dir.clone() + "/";
            let dir_size: u64 = files
                .iter()
                .filter(|(file, _)| file.starts_with(&dir1))
                .map(|(_, size)| *size)
                .sum();
            dir_size
        });

        //part1
        let p1 = dir_sizes.clone().filter(|x| x <= &100_000).sum();

        let root_size: u64 = files.values().map(|size| *size).sum();
        let unused = 70000000 - root_size;
        let required = 30000000 - unused;

        // part2
        let p2 = dir_sizes
            .clone()
            .filter(|x| x >= &required)
            .min()
            .expect("a minimum");

        Ok((p1, p2))
    }
}
