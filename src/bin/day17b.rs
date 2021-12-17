


fn steps(
    dx_init: isize,
    dy_init: isize,
    target_x: (isize, isize),
    target_y: (isize, isize),
) -> Option<Vec<(isize, isize)>> {
    let mut steps = vec![];

    let (mut dx, mut dy, mut x, mut y) = (dx_init, dy_init, 0, 0);
    loop {
        x += dx;
        y += dy;
        steps.push((x, y));
        dy-=1;
        if dx > 0 { dx -= 1 };
        if dx < 0 { panic!() };

        if y >= target_y.0 && y <= target_y.1
            && x >= target_x.0 && x <= target_x.1 {
            return Some(steps);
        }
        if y < target_y.0 { return None; }
    }
}

fn init_positions_that_reach_target(target_x: (isize, isize), target_y: (isize, isize)) -> usize {
    (-1000..1000)
        .flat_map(|y| (0..1000).map(move |x| (x, y)) )
        .filter_map(|(dx, dy)| steps(dx, dy, target_x, target_y))
        .count()
}


fn main() {
    // println!("{}", init_positions_that_reach_target((20, 30), (-10, -5)));

    // 2183 is too low
    println!("{}", init_positions_that_reach_target((150, 193), (-136, -86)));

    // dbg!(steps(2, -5, -10));
}



