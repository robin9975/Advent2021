

use std::collections::HashMap;

struct Image {
    image: HashMap<(isize, isize), char>,
    outer: char
}


fn diff_iter() -> impl Iterator<Item=(isize, isize)> {
    (-1..=1)
        .flat_map(|x| {
            (-1..=1).map(move |y| (x, y))
        })
}


fn enhance_image(alg: &[char], image: &Image) -> Image {
    let min_x = image.image.keys().map(|x| x.0).min().unwrap() - 1;
    let max_x = image.image.keys().map(|x| x.0).max().unwrap() + 1;
    let min_y = image.image.keys().map(|x| x.1).min().unwrap() - 1;
    let max_y = image.image.keys().map(|x| x.1).max().unwrap() + 1;

    let mut new = HashMap::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let d = diff_iter()
                .map(|(dx, dy)| image.image.get(&(x + dx, y + dy)).unwrap_or(&image.outer))
                .map(|c| match c {
                    '.' => '0',
                    '#' => '1',
                    _ => unreachable!()
                })
                .collect::<String>();
            let index = usize::from_str_radix(&d, 2).unwrap();
            let value = alg[index];
            new.insert((x, y), value);
        }
    }

    let outer = match image.outer {
        '.' => alg[0],
        '#' => alg[alg.len() - 1],
        _ => unreachable!()
    };

    Image {
        image: new,
        outer,
    }
}


fn parse_input(inp: &str) -> (Vec<char>, Image) {
    let mut inp = inp.split_terminator("\n\n");
    let alg = inp.next().unwrap().chars().collect();

    let image = inp.next().unwrap().split_terminator("\n")
        .enumerate()
        .flat_map(|(x, row)| {
            row.chars()
                .enumerate()
                .map(move |(y, c)| ((x as isize, y as isize), c))
        })
        .collect();

    (alg, Image { image, outer: '.' })
}

fn main() {
    // 5813 == too high
    // 5344 == too low

    let inp = std::fs::read_to_string("input/day20").unwrap();
    let (alg, mut image) = parse_input(&inp);

    for _ in 0..50 {
        image = enhance_image(&alg, &image);
    }

    let count = image.image.values().filter(|c| c == &&'#').count();
    println!("Part 1: {}", count);
}
