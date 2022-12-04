use std::fs;
use std::io;
use std::io::BufRead;
use std::error::Error;

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
    
    let mut full_contains: u32 = 0;
    for pair in reader.lines() {
        let pair: Vec<Vec<u32>> = pair?
            .split(',')
            .map(|s| -> Vec<u32> { s
                .split('-')
                .map(|idx| idx.parse::<u32>())
                .filter_map(|idx| idx.ok())
                .collect()})
            .collect();
        if pair.len() != 2 || pair[0].len() != 2 || pair[1].len() != 2 {
            Err("unexpected input line size")?;
        }
        if (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1]) || (pair[1][0] >= pair[0][0] && pair[1][1] <= pair[0][1]) {
            full_contains += 1;
        }

    }
    return Ok(full_contains);
}

fn pt2() -> Result<u32, Box<dyn Error>> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut overlaps: u32 = 0;
    for pair in reader.lines() {
        let pair: [[u32; 2]; 2] = pair?
            .split(',')
            .map(|s| -> [u32; 2] { s
                .split('-')
                .map(|idx| idx.parse::<u32>())
                .filter_map(|idx| idx.ok())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap()
            })
            .collect::<Vec<[u32;2]>>()
            .try_into()
            .unwrap();

        let within = |id: u32, range: &[u32; 2]| -> bool { id >= range[0] && id <= range[1] };
        if within(pair[0][0], &pair[1]) || within(pair[0][1], &pair[1]) || within(pair[1][0], &pair[0]) || within(pair[1][1], &pair[0]) {
            overlaps += 1;
        }

    }
    return Ok(overlaps);
}