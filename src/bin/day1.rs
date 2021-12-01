
use std::{ io::{self, BufRead}, fs::File };

fn get_values() -> Vec<usize> {
    let f = File::open("input/day1.txt").unwrap();
    let values = io::BufReader::new(f)
        .lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    values
}

fn part1() {
    let values = get_values();
    println!("{}", values.len());

    let increasing = (1..values.len())
        .map(|x| values[x] > values[x-1])
        .filter(|x| *x)
        .collect::<Vec<_>>();
    println!("{}", increasing.len());
}

fn part2() {
    let values = get_values();

    let windows = (0..values.len() - 2)
        .map(|x| values[x] + values[x + 1] + values[x + 2])
        .collect::<Vec<_>>();
    println!("{}", values.len());

    let increasing = (1..windows.len())
        .map(|x| windows[x] > windows[x-1])
        .filter(|x| *x)
        .collect::<Vec<_>>();
    println!("{}", increasing.len());
}


fn main() {
    part1();
    part2();
}
