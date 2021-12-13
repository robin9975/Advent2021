
use std::fmt::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct PointSystem {
    points: HashSet<(usize, usize)>,
    folds: Vec<(char, usize)>
}

impl PointSystem {

    fn fold_first(&mut self) {
        let fold = self.folds.remove(0);
        let points_to_remove;
        {
            points_to_remove = self.points.iter()
                .filter(|(x, y)| {
                    match fold.0 {
                        'x' => x > &fold.1,
                        'y' => y > &fold.1,
                        _ => panic!()
                    }
                })
                .cloned()
                .collect::<Vec<(usize, usize)>>();
        }

        for (x, y) in points_to_remove {
            self.points.remove(&(x, y));
            match fold.0 {
                'x' => self.points.insert(
                    (fold.1 + fold.1 - x, y)
                ),
                'y' => self.points.insert(
                    (x, fold.1 + fold.1 - y)
                ),
                _ => panic!()
            };
        }
    }

    fn fold(&mut self) {
        while !self.folds.is_empty() {
            self.fold_first();
            println!("{}", &self);
            print!("\n\n\n\n\n");
        }
    }
}

impl Display for PointSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let max_x = self.points.iter().map(|(x, y)| x).max().unwrap();
        let max_y = self.points.iter().map(|(x, y)| y).max().unwrap();

        for y in 0..=*max_y {
            for x in 0..=*max_x {
                if self.points.contains(&(x, y)) {
                    write!(f, "#");
                } else {
                    write!(f, " ");
                }
            }
            writeln!(f, "");
        }

        Ok(())
    }
}

fn parse_input(input: &str) -> PointSystem {
    let mut points = HashSet::new();
    let mut line_iter = input.lines();

    for line in &mut line_iter {
        if line == "" { break; }
        let mut y = line.split_terminator(",");
        points.insert((
            y.next().unwrap().parse().unwrap(),
            y.next().unwrap().parse().unwrap()
        ));
    }

    let mut folds = Vec::new();
    for line in line_iter {
        dbg!(&line);
        let mut split = line.split_whitespace().nth(2).unwrap().split_terminator("=");

        folds.push((
            split.next().unwrap().chars().next().unwrap(),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    PointSystem {
        points,
        folds
    }
}

fn main() {
    let input = std::fs::read_to_string("input/day13").unwrap();
    let mut system = parse_input(&input);
    assert_eq!(system.folds.len(), 12);

    system.fold_first();
    println!("{}", system.points.len());


    system.fold();
    println!("{}", system);
}


#[test]
fn test_demo() {
    let mut system = parse_input("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5");
    assert_eq!(system.points.len(), 18);
    assert_eq!(system.folds.len(), 2);

    system.fold_first();
    assert_eq!(system.points.len(), 17);

}

