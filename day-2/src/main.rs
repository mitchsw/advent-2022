use std::fs;
use std::io;
use std::io::BufRead;
use std::error::Error;
use std::str::FromStr;
use strum_macros::EnumString;

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
    
    // Forcing myself to use 1-line iterators without any collect. This isn't clean :)
    let score = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| [line.as_bytes()[0]-b'A', line.as_bytes()[2] - b'X'])
        .map(|r| (((r[0]+1)%3 == r[1]) as u8)*6 + ((r[0]==r[1]) as u8)*3 + (r[1]+1))
        .map(|score| score as u32)
        .sum();

    return Ok(score);
}

// #############################################
// For part 2, I want to play around with
// parsing into well-named enums/structs
// #############################################

#[derive(Debug, PartialEq, EnumString)]
enum Action {
    #[strum(serialize = "A")]
    Rock,
    #[strum(serialize = "B")]
    Paper,
    #[strum(serialize = "C")]
    Scissors
}

impl Action { 
    fn score(&self) -> u32 { 
        match self {
            Action::Rock=>return 1,
            Action::Paper=>return 2,
            Action::Scissors=>return 3,
        }
    }

    // returns the action that beats self
    fn beaten_by(&self) -> Action { 
        match self {
            Action::Rock=>return Action::Paper,
            Action::Paper=>return Action::Scissors,
            Action::Scissors=>return Action::Rock,
        }
    }

    // returns the action that self beats
    fn beats(&self) -> Action { 
        match self {
            Action::Rock=>return Action::Scissors,
            Action::Paper=>return Action::Rock,
            Action::Scissors=>return Action::Paper,
        }
    }
}

#[derive(Debug, PartialEq, EnumString)]
enum Outcome {
    #[strum(serialize = "X")]
    Lose,
    #[strum(serialize = "Y")]
    Draw,
    #[strum(serialize = "Z")]
    Win
}

impl Outcome { 
    fn score(&self) -> u32 { 
        match self {
            Outcome::Lose=>return 0,
            Outcome::Draw=>return 3,
            Outcome::Win=>return 6,
        }
    }
}


fn pt2() -> Result<u32, Box<dyn Error>> {
    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    
    let mut score: u32 = 0;
    for line in reader.lines() {
        let line = line?;
        let opponent = Action::from_str(&line[..1])?;
        let outcome = Outcome::from_str(&line[2..])?;

        let hand = match outcome {
            Outcome::Lose=> opponent.beats(),
            Outcome::Draw=> opponent,
            Outcome::Win=>opponent.beaten_by(),
        };
        score += outcome.score() + hand.score();
    }

    return Ok(score);
}