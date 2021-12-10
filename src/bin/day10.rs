
use std::{ fs::File, io::BufReader, io::BufRead };


#[derive(Clone)]
enum LResult {
    Incomplete(Vec<char>),
    Invalid(char, usize)
}

fn check_line(line: String) -> LResult {
    let mut stack = vec![];
    for (index, c) in line.chars().enumerate() {
        match c {
            '(' | '[' | '<' | '{' => {
                stack.push(c);
            },
            ')' | ']' | '>' | '}' => {
                let start_char = match c {
                    ')' => '(',
                    ']' => '[',
                    '>' => '<',
                    '}' => '{',
                    _ => panic!("Unsupported character")
                };
                // assumes that closing chars are subsequent u32's
                if stack.pop().unwrap() != start_char {
                    return LResult::Invalid(c, index)
                }
            },
            _ => panic!("Unsupported character")
        }
    }

    LResult::Incomplete(stack)
}


fn autocomplete(mut input: LResult) -> Option<String> {
    if let LResult::Incomplete(mut chars) = input {
        chars.reverse();
        Some(chars.into_iter()
            .map(|c| {
                match c {
                    '(' => ')',
                    '[' => ']',
                    '<' => '>',
                    '{' => '}',
                    _ => panic!("Unsupported character")
                }
            })
            .collect::<String>()
        )
    } else {
        None
    }
}

fn autocomplete_score(input: String) -> usize {
    input.chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => todo!()
        })
        .fold(0, |acc, cscore| {
            acc * 5 + cscore
        })
}

fn score_invalid(input: LResult) -> usize {
    if let LResult::Invalid(c, pos) = input {
        return match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0
        }
    }

    0
}

fn main() {
    let filename = "input/day10";
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(f).lines();

    let results = lines
        .into_iter()
        .map(|x| x.unwrap())
        .map(check_line)
        .collect::<Vec<_>>();

    // Part 1
    let score = results.clone()
        .into_iter()
        .map(score_invalid)
        .sum::<usize>();
    println!("Part 1: {}", score);


    // Part 2
    let mut scores = results.into_iter()
        .filter_map(autocomplete)
        .map(autocomplete_score)
        .collect::<Vec<_>>();
    scores.sort();
    dbg!(scores[scores.len() / 2]);
}
