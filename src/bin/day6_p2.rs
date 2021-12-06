
use std::collections::HashMap;

type Fish = HashMap<usize, usize>;



fn simulate_day(fish: &Fish) -> Fish {
    vec![
        (0, *fish.get(&1).unwrap_or(&0)),
        (1, *fish.get(&2).unwrap_or(&0)),
        (2, *fish.get(&3).unwrap_or(&0)),
        (3, *fish.get(&4).unwrap_or(&0)),
        (4, *fish.get(&5).unwrap_or(&0)),
        (5, *fish.get(&6).unwrap_or(&0)),
        (6, *fish.get(&0).unwrap_or(&0) + *fish.get(&7).unwrap_or(&0)),
        (7, *fish.get(&8).unwrap_or(&0)),
        (8, *fish.get(&0).unwrap_or(&0))
    ].into_iter().collect()
}

fn simulate_n_days(days: usize, fish: Fish) -> Fish {
    let mut f = fish;
    for n in 0..days {
        f = simulate_day(&f);
        println!("Day {}", n + 1);
    }
    f
}


fn main() {
    let content = std::fs::read_to_string("input/day6")
        .unwrap();
    let mut numbers = content
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    let mut fish: Fish = HashMap::new();
    for n in numbers.iter() {
        *fish.entry(*n).or_insert(0) += 1;
    }

    let fish = simulate_n_days(256, fish);
    println!(
        "{}",
        fish.values().sum::<usize>(),
    );
}

#[test]
fn test_demo() {
    let numbers = vec![3, 4, 3, 1, 2];

    let mut fish: Fish = HashMap::new();
    for n in numbers.iter() {
        *fish.entry(*n).or_insert(0) += 1;
    }

    let result = simulate_n_days(80, fish);

    assert_eq!(
        result.values().sum::<usize>(),
        5934
    );
}
