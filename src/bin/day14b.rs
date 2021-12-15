
use std::collections::HashMap;


type Pair = [char; 2];

struct System {
    polymer: HashMap<Pair, usize>,
    last_char: char,
    pair_insertions: HashMap<Pair, [Pair; 2]>,
}

impl System {
    fn step(&mut self) {
        let mut new_counts = HashMap::new();
        for (pair, count) in self.polymer.iter() {
            let inserts = self.pair_insertions[pair];
            *new_counts.entry(inserts[0]).or_insert(0) += count;
            *new_counts.entry(inserts[1]).or_insert(0) += count;
        }
        self.polymer = new_counts;
    }

    fn score(&self) -> usize {
        let mut counts = HashMap::new();

        for (p, c) in self.polymer.iter() {
            *counts.entry(p[0]).or_insert(0) += c;
        }
        *counts.entry(self.last_char).or_insert(0) += 1;

        let mut v : Vec<(&char, &usize)> = counts.iter().collect();
        v.sort_by_key(|x| x.1);
        v.last().unwrap().1 - v[0].1
    }
}

fn parse_input(input: &str) -> System {
    let mut lineit = input.lines();

    let polymer = lineit.next().unwrap().to_string();
    let pchars = polymer.chars().collect::<Vec<_>>();
    let mut polymer_pairs = vec![];
    for i in 0..(polymer.len() - 1) {
        polymer_pairs.push([
            pchars[i],
            pchars[i + 1],
        ]);
    }

    let mut pair_counts = HashMap::new();
    for pair in polymer_pairs.iter() {
        *pair_counts.entry(*pair).or_insert(0) += 1;
    }

    let last_char = *pchars.last().unwrap();

    lineit.next();
    let pair_insertions = lineit
        .map(|rule| {
            let mut rchars = rule.chars().collect::<Vec<_>>();
            (
                [rchars[0], rchars[1]],
                [
                    [rchars[0], rchars[6]],
                    [rchars[6], rchars[1]],
                ]
            )
        })
        .collect();

    System {
        polymer: pair_counts,
        last_char,
        pair_insertions
    }
}


fn main() {
    let content = std::fs::read_to_string("input/day14").unwrap();
    let mut system = parse_input(&content);

    for i in 0..40 {
        println!("Step {}", i);
        system.step();
        // println!("{}", system.polymer);
    }
    println!("Score: {}", system.score());
}

#[test]
fn test_demo()  {
    let content = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let mut system = parse_input(content);
//    assert_eq!(system.polymer, "NNCB");
    // assert_eq!(system.pair_insertions["CH"], ["CB", "BH"]);

    for _ in 0..10 {
        system.step();
        // println!("{}", system.polymer);
    }
    assert_eq!(system.score(), 1588);
}

