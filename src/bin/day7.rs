

fn step_costs(max: isize) -> Vec<isize> {
    let mut last = 0;
    (0..max)
        .map(|x| {
            let value = last + x;
            last = value;
            value
        })
        .collect()
}


fn fuel_need(positions: &[isize], target: isize) -> isize {
    positions.iter()
        .map(|x| (x - target).abs())
        .sum()
}

fn fuel_actual_need(
    positions: &[isize],
    target: isize,
    step_costs: &[isize]
) -> isize {
    positions.iter()
        .map(|x| step_costs[(x - target).abs() as usize])
        .sum()
}

fn min_fuel_position(positions: &[isize]) -> (isize, isize) {
    let max = positions.iter().max().unwrap();
    (0..*max)
        .map(|x| (x, fuel_need(positions, x)))
        .min_by_key(|x| x.1)
        .unwrap()
}

fn actual_min_fuel_position(positions: &[isize]) -> (isize, isize) {
    let max = positions.iter().max().unwrap();
    let steps = step_costs(*max + 1);
    (0..*max)
        .map(|x| (x, fuel_actual_need(positions, x, &steps)))
        .min_by_key(|x| x.1)
        .unwrap()
}

fn main() {
    let content = std::fs::read_to_string("input/day7")
        .unwrap();

    let numbers = content
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<isize>>();

    println!("part 1: {:?}", min_fuel_position(&numbers));
    println!("part 2: {:?}", actual_min_fuel_position(&numbers));
}


#[test]
fn test_demo() {
    let positions = vec![
        16, 1, 2, 0, 4, 2, 7, 1, 2, 14
    ];
    assert_eq!(fuel_need(&positions, 2), 37);
    assert_eq!(fuel_need(&positions, 3), 39);
    assert_eq!(fuel_need(&positions, 1), 41);
    assert_eq!(fuel_need(&positions, 10), 71);

    assert_eq!(min_fuel_position(&positions), (2, 37));
}


#[test]
fn test_p2() {
    let positions = vec![
        16, 1, 2, 0, 4, 2, 7, 1, 2, 14
    ];
    let steps = step_costs(17);
    assert_eq!(fuel_actual_need(&positions, 2, &steps), 206);
    //assert_eq!(fuel_need(&positions, 3), 39);
    //assert_eq!(fuel_need(&positions, 1), 41);
    //assert_eq!(fuel_need(&positions, 10), 71);

    assert_eq!(actual_min_fuel_position(&positions), (5, 168));
}
