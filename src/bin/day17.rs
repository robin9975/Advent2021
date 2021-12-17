


fn steps(
    dy_init: isize,
    target_min: isize,
    target_max: isize
) -> Option<Vec<isize>> {
    let mut steps = vec![];

    let mut dy = dy_init;
    let mut y: isize = 0;
    loop {
        y += dy;
        steps.push(y);
        dy-=1;

        if y <= target_min && y >= target_max { return Some(steps); }
        if y < target_max { return None; }
    }
}

fn get_optimal_y(target_min: isize, target_max: isize) -> isize {
    (0..100)
        .filter_map(|dy_init| steps(dy_init, target_min, target_max))
        .flat_map(|y_positions| y_positions)
        .max()
        .unwrap()
}

fn main() {
    println!("Highest y: {}", get_optimal_y(-86, -136));

    // dbg!(steps(2, -5, -10));
}



