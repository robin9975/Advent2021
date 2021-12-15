
use std::collections::HashMap;


struct System {
    polymer: String,
    pair_insertions: HashMap<String, String>,
}

impl System {
    fn step(&mut self) {
        let mut new = String::new();
        for i in 0..(self.polymer.len() - 1) {
            let pair = &self.polymer[i..=(i+1)];
            let insert = self.pair_insertions.get(pair)
                .expect("Invalid pair");

            new.push_str(&pair[0..1]);
            new.push_str(insert);

            if i == self.polymer.len() - 2 {
                new.push_str(&pair[1..2]);
            }
        }

        self.polymer = new;
    }

    fn score(&self) -> usize {
        let mut chars = self.polymer.chars()
            .collect::<Vec<char>>();
        chars.sort();

        let mut counts : Vec<(char, usize)> = vec![];
        let it = chars.iter();
        let splits = it.enumerate()
            .filter_map(|(index, c)| {
                if index + 1 < chars.len() && chars[index + 1] == *c {
                    None
                } else {
                    Some((index, *c))
                }
            })
            .collect::<Vec<(usize, char)>>();

        counts.push((splits[0].1, splits[0].0 + 1));
        for i in 1..splits.len() {
            counts.push((splits[i].1, splits[i].0 - splits[i-1].0));
        }
        counts.sort_by_key(|x| x.1);

        counts[counts.len() - 1].1 - counts[0].1
    }
}

fn parse_input(input: &str) -> System {
    let mut lineit = input.lines();

    let polymer = lineit.next().unwrap().to_string();
    lineit.next();
    let pair_insertions = lineit
        .map(|rule| {
            (rule[0..2].to_string(), rule[6..7].to_string())
        })
        .collect();

    System {
        polymer,
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
    assert_eq!(system.polymer, "NNCB");
    assert_eq!(system.pair_insertions["CH"], "B");

    for _ in 0..10 {
        system.step();
        println!("{}", system.polymer);
    }
    assert_eq!(system.score(), 1588);
}

