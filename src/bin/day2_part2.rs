
use std::str::FromStr;
use std::{ io::{self, BufRead}, fs::File };



#[derive(Default, Debug, PartialEq)]
struct Position {
    x: usize,
    depth: usize,
    aim: usize,
}

#[derive(Debug, PartialEq)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize)
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_whitespace();
        let cmd = splitted.next().unwrap();
        let value = splitted.next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Ok(match cmd {
            "forward" => Command::Forward(value),
            "up" => Command::Up(value),
            "down" => Command::Down(value),
            _ => Err("Invalid command".to_string())?
        })
    }
}

impl std::ops::Add<Command> for Position {
    type Output = Position;

    fn add(mut self, rhs: Command) -> Self::Output {
        match rhs {
            Command::Forward(x) => {
                self.x += x;
                self.depth += self.aim * x;
            },
            Command::Up(x) => self.aim -= x,
            Command::Down(x) => self.aim += x,
        }
        self
    }
}

fn get_values() -> Vec<Command> {
    let f = File::open("input/day2.txt").unwrap();
    let values = io::BufReader::new(f)
        .lines()
        .map(|x| x.unwrap().parse::<Command>().unwrap())
        .collect::<Vec<_>>();
    values
}

fn main() {
    let mut pos = Position::default();

    for cmd in get_values().into_iter() {
        pos = pos + cmd;
    }
    println!("{:?}", pos);
    println!("x * depth: {}", pos.x * pos.depth);
}



#[test]
fn test_parse_command() {
    assert_eq!(
        "forward 10".parse::<Command>().unwrap(),
        Command::Forward(10)
    )
}
