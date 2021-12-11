
use std::{ fs::File, io::BufReader, io::BufRead };

struct Grid {
    levels: Vec<Vec<i32>>
}

impl std::fmt::Display for Grid  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in self.levels.iter() {
            for value in row.iter() {
                write!(f, "{}", value);
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

impl Grid {
    fn all_zeroes(&self) -> bool {
        for x in 0..10 {
            for y in 0..10 {
                if self.levels[x][y] != 0 { return false; }
            }
        }
        true
    }

    fn step(&mut self) -> usize {
        // Increase all levels by 1
        for x in 0..10 {
            for y in 0..10 {
                self.levels[x][y] += 1;
            }
        }

        let mut flash_count = 0;
        for x in 0..10 {
            for y in 0..10 {
                if self.levels[x][y] > 9 {
                    flash_count += self.flash_location(x, y);
                }
            }
        }

        flash_count
    }

    fn flash_location(&mut self, x: usize, y: usize) -> usize {
        let mut flash_count = 0;

        self.levels[x][y] = 0;

        let delta: Vec<(i32, i32)> = vec![
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];
        for (dx, dy) in delta.iter() {
            let tx = x as i32 + dx;
            let ty = y as i32 + dy;

            if (tx < 0) | (tx > 9) { continue; }
            if (ty < 0) | (ty > 9) { continue; }
            let tx = tx as usize;
            let ty = ty as usize;

            // already flashed, because the non-flashed
            // always have at least level 1
            if self.levels[tx][ty] == 0 { continue; }

            self.levels[tx][ty] += 1;
            if self.levels[tx][ty] > 9 {
                flash_count += self.flash_location(tx, ty);
            }
        }

        1 + flash_count
    }
}


fn main() {
    let filename = "input/day11";
    let f = File::open(filename).unwrap();
    let lines = BufReader::new(f).lines();

    let levels = lines
        .into_iter()
        .map(|x| x.unwrap())
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect::<Vec<Vec<i32>>>();

    let mut grid = Grid { levels };

    let mut sum = 0;
    for step in 1..=1000 {
        let count = grid.step();
        sum += count;
        println!("Step {}: {}; total: {}", step, count, sum);

        if grid.all_zeroes() {
            println!("Synchronized!");
            println!("{}", grid);
            break;
        }
    }
}


