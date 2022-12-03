use std::fs;
use std::io;
use std::io::BufRead;
use std::error::Error;
use std::collections::HashSet;
use itertools::Itertools;

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
    
    let mut pri_sum: u32 = 0;
    for line in reader.lines() {
        let mut bag1 = HashSet::<char>::new();
        let mut bag2 = HashSet::<char>::new();
        let line = line?;
        for (i, c) in line.chars().enumerate() {
            if i < line.len()/2 {
                bag1.insert(c);
            } else {
                bag2.insert(c);
            }
        }
        let a: Vec<&char> = bag1.intersection(&bag2).collect();
        if a.len() != 1 {
            Err("unexpected intersection")?;
        }
        let c = *a[0];
        pri_sum += char_pri(c)?;
    }
    return Ok(pri_sum);
}

fn pt2() -> Result<u32, Box<dyn Error>> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut pri_sum: u32 = 0;
    for group in &reader.lines().chunks(3) {
        let set = group
            .filter_map(|line| line.ok())
            .map(|line| HashSet::<char>::from_iter(line.chars()))
            .reduce(|accum, set| accum.intersection(&set).copied().collect())
            .ok_or("no lines?")?;
        if set.len() != 1 {
            Err("unexpected intersection")?;
        }
        let c = *set.iter().next().unwrap();
        pri_sum += char_pri(c)?;
    }
    return Ok(pri_sum);
}

fn char_pri(c: char) -> Result<u32, Box<dyn Error>>  {
    if c.is_ascii_lowercase() {
        return Ok(((c as u8) - b'a' + 1) as u32);
    } else if c.is_ascii_uppercase() {
        return Ok(((c as u8) - b'A' + 27) as u32);
    }
    Err("unexpected rucksack char")?
}