
use std::{ fs::File, io::BufReader, io::BufRead };
use std::collections::{ HashSet, HashMap };


#[derive(Debug)]
struct Row {
    patterns: Vec<String>,
    output: Vec<String>
}

impl Row {

    fn count_1478_in_output(&self) -> usize {
        self.output
            .iter()
            .filter(|x| {
                x.len() == 2 // number 1
                    || x.len() == 4 // number 4
                    || x.len() == 3 // number 7
                    || x.len() == 7 // number 8
            })
            .count()
    }

    ///
    /// Create a mapping that describes which 7-segment
    /// position is described by which number.
    ///
    ///      0000
    ///     1    2
    ///     1    2
    ///      3333
    ///     4    5
    ///     4    5
    ///      6666
    ///
    ///
    fn solve_mapping(&mut self) -> Vec<char> {
        let mut possibilities = (0..7)
            .map(|x| vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']
                .into_iter().collect::<HashSet<_>>()
                )
            .collect::<Vec<_>>();

        self.patterns.sort_by_key(|x| x.len());

        for pattern in self.patterns.iter() {
            let patternset: HashSet<char> = pattern.chars().collect();
            match pattern.len() {
                // number 1
                2 => {
                    possibilities[2] = possibilities[2].intersection(&patternset).cloned().collect();
                    possibilities[5] = possibilities[5].intersection(&patternset).cloned().collect();

                    for i in 0..7 {
                        if i == 2 || i == 5 { continue; }
                        possibilities[i] = possibilities[i]
                            .difference(&possibilities[2])
                            .cloned()
                            .collect();
                    }
                },
                // number 7
                3 => {
                    // 2 and 5 are the same as the one before, so we can ignore those
                    possibilities[0] = patternset
                        .difference(&possibilities[2]) // 2 and 5 should be the same after the above
                        .cloned()
                        .collect();

                    assert!(possibilities[0].len() == 1);

                    for i in 1..7 {
                        possibilities[i] = possibilities[i]
                            .difference(&possibilities[0])
                            .cloned()
                            .collect();
                    }
                },
                // number 4
                4 => {
                    // possibilities 2 and 5 remain the same, no additional
                    // information to be gotten here

                    let options_pos_1_3: HashSet<char> = patternset
                        .difference(&possibilities[2])
                        .cloned()
                        .collect();

                    assert_eq!(options_pos_1_3.len(), 2);

                    possibilities[1] = options_pos_1_3.clone();
                    possibilities[3] = options_pos_1_3.clone();

                    for i in 0..7 {
                        if i == 1 || i == 3 { continue; }
                        possibilities[i] = possibilities[i]
                            .difference(&options_pos_1_3)
                            .cloned()
                            .collect();
                    }

                },
                // Either 2, 3, or 5
                5 => {
                    // Number 2 is position 1 or 5 aren't a possibility
                    if patternset.intersection(&possibilities[1]).count() == 0
                        || patternset.intersection(&possibilities[5]).count() == 0 {
                        println!("It is a 2!")
                    }
                    // Number 3 if position 1 or 4 aren't a possibility
                    if patternset.intersection(&possibilities[1]).count() == 0
                        || patternset.intersection(&possibilities[4]).count() == 0 {
                        println!("It is a 3!")
                    }
                    // Number 5 if position 1 or 4 aren't a possibility
                    if patternset.intersection(&possibilities[2]).count() == 0
                        || patternset.intersection(&possibilities[4]).count() == 0 {
                        println!("It is a 5!")
                    }
                },
                // Either 0, 6 or 9
                6 => {
                    if possibilities[4].difference(&patternset).count() == 1 {
                        possibilities[4] = possibilities[4]
                            .difference(&patternset)
                            .cloned()
                            .collect();

                        for i in 0..7 {
                            if i == 4 { continue }
                            possibilities[i] = possibilities[i]
                                .difference(&possibilities[4])
                                .cloned()
                                .collect();
                        }
                    }

                    // if all possibilities of position 3 aren't included, it must be 0
                    if possibilities[3].difference(&patternset).count() == 1 {
                        possibilities[3] = possibilities[3]
                            .difference(&patternset)
                            .cloned()
                            .collect();

                        for i in 0..7 {
                            if i == 3 { continue }
                            possibilities[i] = possibilities[i]
                                .difference(&possibilities[3])
                                .cloned()
                                .collect();
                        }
                    }

                    // if all possibilities of position 2 aren't included, it must be 6
                    if possibilities[2].difference(&patternset).count() == 1 {
                        possibilities[2] = possibilities[2]
                            .difference(&patternset)
                            .cloned()
                            .collect();

                        for i in 0..7 {
                            if i == 2 { continue }
                            possibilities[i] = possibilities[i]
                                .difference(&possibilities[2])
                                .cloned()
                                .collect();
                        }
                    }
                },
                // number 7
                7 => { continue }, // since it doesn't give any information
                _ => {
                    panic!("Invalid pattern length");
                }
            }
        }


        for p in possibilities.iter() {
            assert_eq!(p.len(), 1);
        }

        possibilities.into_iter()
            .map(|mut x| x.drain().next().unwrap())
            .collect()
    }
}

///      0000
///     1    2
///     1    2
///      3333
///     4    5
///     4    5
///      6666
fn number_map(mapping: &[char]) -> HashMap<String, String> {
    let number_positions = vec![
        vec![0, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6 ],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6]
    ];
    let mut map = HashMap::new();
    for (n, positions) in number_positions.iter().enumerate() {
        let mut key = positions.iter()
            .map(|x| mapping[*x])
            .collect::<Vec<char>>();
        key.sort();
        map.insert(
            key.into_iter().collect(),
            format!("{}", n),
        );
    }

    map
}

fn resolve_numbers(mapping: &[char], input: &str) -> usize {
    let mut chars = input.chars().collect::<Vec<char>>();
    chars.sort();
    let value = chars.into_iter().collect::<String>();

    let map = number_map(mapping);
    map.get(&value)
        .unwrap()
        .parse()
        .expect("Invalid map value")
}

fn parse_input(filename: &str) -> Vec<Row> {
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(f).lines();

    let mut rows = vec![];

    for line in lines {
        let line = line.unwrap();
        let mut splitted = line.split(" | ")
            .map(|x| {
                x.split_whitespace()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            })
        ;

        rows.push(Row {
            patterns: splitted.next().unwrap(),
            output: splitted.next().unwrap(),
        });
    }

    rows
}

fn part1() {
    let rows = parse_input("input/day8");
    let count: usize = rows.iter()
        .map(|x| x.count_1478_in_output())
        .sum();
    println!("Part 1: {}", count);
}

fn part2() {
    let mut sum = 0;

    let mut rows = parse_input("input/day8");
    for row in rows.iter_mut() {
        let mapping = row.solve_mapping();
        let numbers = row.output.iter()
            .map(|output| resolve_numbers(&mapping, output))
            .collect::<Vec<usize>>();

        sum += 1000 * numbers[0] +
            100 * numbers[1] +
            10 * numbers[2] +
            numbers[3];
    }

    println!("Part 2: {}", sum);
}

fn main() {
    part1();
    part2();
}


#[test]
fn test_p1_demo() {
    let rows = parse_input("input/day8_test");
    let count: usize = rows.iter()
        .map(|x| x.count_1478_in_output())
        .sum();
    assert_eq!(count, 26);
}


#[test]
fn test_p2_demo() {
    let mut rows = parse_input("input/day8_test");

    for row in rows.iter_mut() {
        let mapping = row.solve_mapping();
        let numbers = row.output.iter()
            .map(|output| resolve_numbers(&mapping, output))
            .collect::<Vec<usize>>();

        let result = 1000 * numbers[0] +
            100 * numbers[1] +
            10 * numbers[2] +
            numbers[3];
        println!("Result: {}", result);
    }
}
