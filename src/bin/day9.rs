
use std::{ fs::File, io::BufReader, io::BufRead };

struct Map {
    numbers: Vec<Vec<i32>>
}

#[derive(Debug)]
struct Delta {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
    basin_id: i32
}

impl Delta {
    fn is_lowpoint(&self) -> bool {
        self.top > 0
            && self.bottom > 0
            && self.left > 0
            && self.right > 0
    }
}

impl Map {

    fn get_basins(&self) {
        let mut delta = self.get_delta();

        // set the basin_id of all low points
        self.get_low_points()
            .into_iter()
            .enumerate()
            .for_each(|(index, (x, y, height))| {
                delta[x][y].basin_id = index as i32;
            });

        loop {
            let mut complete = true;
            for x in 0..self.numbers.len() {
                let row = &self.numbers[x];
                for y in 0..row.len() {
                    // already set
                    if delta[x][y].basin_id > -1 { continue; }
                    if row[y] == 9 { continue; }

                    if delta[x][y].top < 0 && delta[x][y - 1].basin_id > -1 {
                        delta[x][y].basin_id = delta[x][y - 1].basin_id;
                        complete = false;
                    }
                    if delta[x][y].bottom < 0 && delta[x][y + 1].basin_id > -1 {
                        delta[x][y].basin_id = delta[x][y + 1].basin_id;
                        complete = false;
                    }
                    if delta[x][y].left < 0 && delta[x - 1][y].basin_id > -1 {
                        delta[x][y].basin_id = delta[x - 1][y].basin_id;
                        complete = false;
                    }
                    if delta[x][y].right < 0 && delta[x + 1][y].basin_id > -1 {
                        delta[x][y].basin_id = delta[x + 1][y].basin_id;
                        complete = false;
                    }
                }
            }

            if complete { break; }
        }


        let mut basin_counts = vec![0; 200];
        for d in delta.iter().flat_map(|x| x.iter()) {
            if d.basin_id == -1 { continue; }
            basin_counts[d.basin_id as usize] += 1;
        }
        basin_counts.sort();
        basin_counts.reverse();
        println!("{:?}", basin_counts);
        println!("Score: {} * {} * {} = {}",
            basin_counts[0],
            basin_counts[1],
            basin_counts[2],
            basin_counts[0] * basin_counts[1] * basin_counts[2]
        );
    }

    fn get_low_points(&self) -> Vec<(usize, usize, i32)> {
        let delta = self.get_delta();

        let mut points : Vec<(usize, usize, i32)> = vec![];

        for x in 0..delta.len() {
            let row = &delta[x];
            for y in 0..row.len() {
                if delta[x][y].is_lowpoint() {
                    points.push((x, y, self.numbers[x][y]));
                }
            }
        }

        points
    }

    fn get_delta(&self) -> Vec<Vec<Delta>> {
        let mut dx : Vec<Vec<Delta>> = Vec::new();

        for x in 0..self.numbers.len() {
            let row : &[i32] = &self.numbers[x];
            let mut row_dx : Vec<Delta> = vec![];


            for y in 0..row.len() {
                let this = self.numbers[x][y];

                row_dx.push(Delta {
                    top: if y > 0 { self.numbers[x][y - 1] - this } else { 9 },
                    bottom: if y + 1 < row.len() {  self.numbers[x][y + 1] - this } else { 9 },
                    left: if x > 0 { self.numbers[x - 1][y] - this } else { 9 },
                    right: if x + 1 < self.numbers.len() { self.numbers[x + 1][y] - this } else { 9 },
                    basin_id: -1,
                })
            }

            dx.push(row_dx);
        }

        dx
    }
}



fn part1() {
    let filename = "input/day9";
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(f).lines();

    let numbers = lines.into_iter()
        .map(|x| x.unwrap())
        .map(|x| x.chars().into_iter()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let map = Map { numbers };
    let score = map.get_low_points()
        .into_iter()
        .map(|(x, y, height)| 1 + height)
        .sum::<i32>();
    println!("Score: {}", score);

    map.get_basins();
}

fn main() {
    part1();
}
