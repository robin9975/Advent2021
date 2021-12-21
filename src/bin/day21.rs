

#[derive(Debug, Clone)]
struct Game {
    p1_position: usize,
    p2_position: usize,
    p1_score: usize,
    p2_score: usize,
    die: Die,
}

impl Game {

    fn new(p1: usize, p2: usize) -> Self {
        Self {
            p1_position: p1 - 1,
            p2_position: p2 - 1,
            p1_score: 0,
            p2_score: 0,
            die: Die { rolled: 0, next: 1 }
        }
    }

    fn step(&mut self) {

        // let d = self.die.clone();

        let p1_throw : usize = self.die.by_ref().take(3).sum();
        self.p1_position = (self.p1_position + p1_throw) % 10;
        self.p1_score += self.p1_position + 1; // we index position 0-9 instead of 1-10

        if self.p1_score >= 1000 { return; }

        let p2_throw : usize = self.die.by_ref().take(3).sum();
        self.p2_position = (self.p2_position + p2_throw) % 10;
        self.p2_score += self.p2_position + 1; // we index position 0-9 instead of 1-10
    }
}

#[derive(Debug, Clone)]
struct Die {
    rolled: usize,
    next: usize,
}
impl Iterator for Die {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.rolled += 1;

        let v = self.next;
        self.next += 1;
        if self.next > 100 {
            self.next = 1;
        }

        Some(v)
    }
}


fn main() {
    let mut game = Game::new(8, 2);
    loop {
        game.step();
        println!("Score: p1: {} -- p2: {}", game.p1_score, game.p2_score);
        if game.p1_score >= 1000 {
            println!("P1 WON!: {} x {} = {}", game.p2_score, game.die.rolled, game.p2_score * game.die.rolled);
            break;
        }
        if game.p2_score >= 1000 {
            println!("P2 WON!: {} x {} = {}", game.p1_score, game.die.rolled, game.p1_score * game.die.rolled);
            break;
        }
    }
}
