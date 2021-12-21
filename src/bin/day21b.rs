


static die_roll_universes:  [(usize, usize); 7] = [ (3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1) ];


#[derive(Debug, Clone)]
struct Game {
    next_turn: usize,
    p1_position: usize,
    p2_position: usize,
    p1_score: usize,
    p2_score: usize,
}

impl Game {

    fn new(p1: usize, p2: usize) -> Self {
        Self {
            next_turn: 1,
            p1_position: p1 - 1,
            p2_position: p2 - 1,
            p1_score: 0,
            p2_score: 0,
        }
    }

    fn has_won(&self) -> Option<usize> {
        if self.p1_score >= 21 { return Some(1); }
        if self.p2_score >= 21 { return Some(2); }
        None
    }

    fn step(&self) -> Vec<(Game, usize)> {
        match self.next_turn {
            1 => self.step_p1(),
            2 => self.step_p2(),
            _ => unreachable!()
        }
    }


    fn step_p1(&self) -> Vec<(Game, usize)> {
        let roll = &die_roll_universes;

        let mut games = Vec::with_capacity(7);
        for (throw, num_universes) in roll {
            let mut g = self.clone();
            g.next_turn = 2;
            g.p1_position = (self.p1_position + throw) % 10;
            g.p1_score += g.p1_position + 1; // we index position 0-9 instead of 1-10
            games.push((g, *num_universes));
        }

        games
    }

    fn step_p2(&self) -> Vec<(Game, usize)> {
        let roll = &die_roll_universes;

        let mut games = Vec::with_capacity(7);
        for (throw, num_universes) in roll {
            let mut g = self.clone();
            g.next_turn = 1;
            g.p2_position = (self.p2_position + throw) % 10;
            g.p2_score += g.p2_position + 1; // we index position 0-9 instead of 1-10
            games.push((g, *num_universes));
        }

        games
    }
}

fn main() {
    let mut global_games = Vec::with_capacity(1000000000);
    global_games.push((Game::new(8, 2), 1));

    let mut p1_games_won = 0;
    let mut p2_games_won = 0;

    loop {
        let (current_game, game_count) = global_games.pop().unwrap();
        let games = current_game.step();

        for (g, c) in games.into_iter() {
            let mut done = true;
            match g.has_won() {
                Some(1) => p1_games_won += c * game_count,
                Some(2) => p2_games_won += c * game_count,
                Some(_) => unreachable!(),
                None => {
                    done = false;
                    global_games.push((g, c * game_count))
                },
            }
        }

        if global_games.len() == 0 {
            break;
        }
    }

    dbg!(p1_games_won, p2_games_won);

}
