

use std::{ io::{self, BufRead}, fs::File };


fn gamma_rate(input: &[usize]) -> usize {
    let mut result: usize = 0;

    for i in 0..5 {
        let mask = 2_usize.pow(i.try_into().unwrap());

        let ones = input.iter()
            .filter(|x| **x & mask != 0)
            .count();

        let even = input.len() & 1 == 0;

        if ones > (input.len() / 2)
            || ((ones == input.len() / 2) && even)
        {
            result = result | mask;
        }
    }

    result
}

fn epsilon_rate(input: &[usize]) -> usize {
    let mut result: usize = 0;

    for i in 0..5 {
        let mask = 2_usize.pow(i.try_into().unwrap());

        let ones = input.iter()
            .filter(|x| **x & mask != 0)
            .count();

        if ones < (input.len() / 2) {
            result = result | mask;
        }
    }

    result
}


fn oxygen_generator_rating(input: &[usize], current_bit: usize) -> usize {
    let mask = 2_usize.pow((5 - current_bit).try_into().unwrap());
    let gamma_rate = gamma_rate(input);

    let result = input.iter()
        .cloned()
        .filter(|x| x & mask == gamma_rate & mask)
        .collect::<Vec<usize>>();
    if result.len() == 1 {
        return result[0];
    }

    oxygen_generator_rating(
        &result,
        current_bit + 1
    )
}


fn co_scrubber_rating(input: &[usize], current_bit: usize) -> usize {
    let mask = 2_usize.pow((5 - current_bit).try_into().unwrap());
    let epsilon_rate = epsilon_rate(input);

    let result = input.iter()
        .cloned()
        .filter(|x| x & mask == epsilon_rate & mask)
        .collect::<Vec<usize>>();
    if result.len() == 1 {
        return result[0];
    }

    oxygen_generator_rating(
        &result,
        current_bit + 1
    )
}


fn get_values(filename: &str) -> Vec<usize> {
    let f = File::open(filename).unwrap();
    let values = io::BufReader::new(f)
        .lines()
        .map(|x| {
            usize::from_str_radix(&x.unwrap(), 2).unwrap()
        })
        .collect::<Vec<_>>();
    values
}


fn main() {
    let values = get_values("input/day3.txt");

    let g = gamma_rate(&values);
    let e = epsilon_rate(&values);

    println!("{} x {} = {}", g, e, g * e);
}


#[test]
fn test_part1() {
    let values = get_values("input/day3_test.txt");

    assert_eq!( gamma_rate(&values), 22);
    assert_eq!( epsilon_rate(&values), 9);
}


#[test]
fn test_oxygen_rating() {
    let values = get_values("input/day3_test.txt");

    assert_eq!( oxygen_generator_rating(&values, 0), 23);
}


#[test]
fn test_co2_rating() {
    let values = get_values("input/day3_test.txt");

    assert_eq!( co2_scrubber_rating(&values, 0), 10);
}
