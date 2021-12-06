

type Fish = Vec<usize>;

fn simulate_day(fish: &mut Fish) {
    let mut new_count = 0;

    for f in fish.iter_mut() {
        if *f == 0 {
            *f = 6;
            new_count += 1;
        } else {
            *f = *f - 1;
        }
    }

    for n in (0..new_count) {
        fish.push(8);
    }
}

fn simulate_n_days(days: usize, fish: &mut Fish) {
    for n in 0..days {
        simulate_day(fish);
        println!("Day {}", n + 1);
    }
}


fn main() {
    let content = std::fs::read_to_string("input/day6")
        .unwrap();
    let mut numbers = content
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<usize>>();

    simulate_n_days(256, &mut numbers);
    println!("{}", numbers.len());
}

#[test]
fn test_demo() {
    let mut fish = vec![3, 4, 3, 1, 2];
    simulate_n_days(80, &mut fish);
    assert_eq!(fish.len(), 5934);
}
