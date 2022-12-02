use std::cmp;
use std::error::Error;
use std::fs;
use std::io;
use std::ops::Add;
use std::cmp::PartialOrd;

fn main() {
    let ans1 = pt1();
    match ans1 {
        Ok(x) => println!("Part 1: {x}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }

    let ans2 = pt2();
    match ans2 {
        Ok(x) => println!("Part 2: {x}"),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn pt1() -> Result<u32, Box<dyn Error>> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let (mut cals, mut max_cals) = (0, 0);
    for line in io::BufRead::lines(reader) {
        let line = line?;
        if line.is_empty() {
            cals = 0;
            continue;
        }
        cals += line.parse::<u32>()?;
        max_cals = cmp::max(cals, max_cals);
    }
    return Ok(max_cals);
}

fn pt2() -> Result<u32, Box<dyn Error>> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut cals: u32 = 0;
    let mut top_cals: Top3<u32> = Top3::new();
    for line in io::BufRead::lines(reader) {
        let line = line?;
        if line.is_empty() {
            top_cals.add(cals);
            cals = 0;
            continue;
        }
        cals += line.parse::<u32>()?;
    }
    top_cals.add(cals);

    return Ok(top_cals.sum());
}


// Top3 let's me play around with Rust basics: structs, generics, traits, etc.
// In theory, I think it is an optimised online algorithm to find the top 3 values.
struct Top3<T> {
    vals: [T; 3] // largest val first
}

impl<T: Default + Copy + Add<Output = T> + PartialOrd> Top3<T> {
    fn new() -> Top3<T> {
        return Top3 { vals: [Default::default(); 3] }
    }

    fn add(&mut self, x: T) {
        if x > self.vals[0] {
            self.vals[2] = self.vals[1];
            self.vals[1] = self.vals[0];
            self.vals[0] = x;
        } else if x > self.vals[1] {
            self.vals[2] = self.vals[1];
            self.vals[1] = x;
        } else if x > self.vals[1] {
            self.vals[2] = x;
        }
    }

    fn sum(&self) -> T {
        return self.vals[0] + self.vals[1] + self.vals[2];
    }
}