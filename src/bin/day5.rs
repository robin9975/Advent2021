
use std::{ fmt, io::{self, BufRead}, fs::File };


#[derive(Debug)]
struct Map {
    count: Vec<Vec<usize>>
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.count.iter() {
            for value in row.iter() {
                if *value == 0 {
                    write!(f, ".");
                } else {
                    write!(f, "{}", value);
                }
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

impl Map {
    fn init(lines: &[Line]) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;

        for l in lines.iter() {
            if l.from.0 > max_x { max_x = l.from.0 };
            if l.to.0 > max_x { max_x = l.to.0 };

            if l.from.1 > max_y { max_y = l.from.1 };
            if l.to.1 > max_y { max_y = l.to.1 };
        }

        Map {
            count: (0..=max_x).map(|x| (0..=max_y).map(|_| 0).collect()).collect()
        }
    }

    fn mark_line(&mut self, line: &Line)  {
        if line.from.0 == line.to.0 {
            self.mark_vertical(line);
        } else if line.from.1 == line.to.1 {
            self.mark_horizontal(line);
        } else {
            self.mark_diagonal(line);
        }
    }

    fn mark_vertical(&mut self, line: &Line) {
        let x = line.from.0;
        let (min, max) = if line.to.1 >= line.from.1 {
            (line.from.1, line.to.1)
        } else {
            (line.to.1, line.from.1)
        };
        for y in (min..=max) {
            self.count[x][y] += 1;
        }
    }

    fn mark_horizontal(&mut self, line: &Line) {
        let y = line.from.1;
        let (min, max) = if line.to.0 >= line.from.0 {
            (line.from.0, line.to.0)
        } else {
            (line.to.0, line.from.0)
        };
        for x in (min..=max) {
            self.count[x][y] += 1;
        }
    }

    fn mark_diagonal(&mut self, line: &Line) {
        let deltax: isize = line.to.0 as isize - line.from.0 as isize;
        let deltay: isize = line.to.1 as isize - line.from.1 as isize;

        let steps = deltax.abs();
        assert_eq!(deltay.abs(), steps);

        for i in 0..=steps {
            let x = line.from.0 as isize + (deltax / steps) * i;
            let y = line.from.1 as isize + (deltay / steps) * i;
            self.count[x as usize][y as usize] += 1;
        }
    }

    fn count_lte(&self, count: usize) -> usize {
        let mut counter = 0;
        for row in self.count.iter() {
            for value in row.iter() {
                if *value >= count {
                    counter += 1;
                }
            }
        }
        counter
    }
}

#[derive(Debug)]
struct Line {
    from: (usize, usize),
    to: (usize, usize)
}

fn parse_input() -> Vec<Line> {
    let f = File::open("input/day5.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut coords = vec![];

    for line in lines {
        let line = line.unwrap();
        let splitted = line.split(" -> ")
            .map(|x| {
                let y =  x.split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (y[0], y[1])
            })
            .collect::<Vec<_>>();

        coords.push(Line {
            from: splitted[0],
            to: splitted[1]
        })
    }

    coords
}


fn main() {
    let lines = parse_input();

    let mut map = Map::init(&lines);
    for l in lines.iter() {
        map.mark_line(l);
    }
    println!("{}", &map);

    println!("Count > 2 = {}", map.count_lte(2));
}
