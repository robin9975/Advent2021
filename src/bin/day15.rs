

use std::fmt::{
    Display,
    Formatter,
    self
};
use std::collections::HashSet;

struct Grid {
    risks: Vec<Vec<i32>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for row in self.risks.iter() {
            for x in row.iter() {
                write!(f, "{}", x)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}


impl From<Vec<Vec<i32>>> for Grid {
    fn from(risks: Vec<Vec<i32>>) -> Self {
        Self {
            risks,
        }
    }
}


#[derive(Debug, Clone)]
struct Node {
    x: usize,
    y: usize,
    cost: i32,
}
impl Node {
    fn new(x: usize, y: usize, cost: i32) -> Self {
        Self { x, y, cost }
    }
}

impl Grid {

    // For part b, extend the grid
    fn expand(&self) -> Self {
        let os = self.risks.len(); // original size
        let size = os * 5;
        let mut grid = vec![];
        for x in 0..size {
            let mut row = vec![];

            for y in 0..size {
                let orig_value = self.risks[x%os][y%os];
                let x_mult = x / os;
                let y_mult = y / os;
                row.push((orig_value + x_mult as i32 + y_mult as i32 - 1) % 9 + 1);
            }

            grid.push(row);
        }

        Self { risks: grid }
    }


    fn h(&self, x: usize, y: usize) -> i32 {
        (self.risks.len() - x + self.risks[0].len() - y) as i32  * 2
    }

    fn lowest_path(&self) -> usize {
        // x, y, current_cost
        let mut open_nodes: Vec<Node> = vec![Node::new(0, 0, 0)];
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        loop {
            // find the lowest next step (node index, target)
            let next_node = open_nodes.iter().enumerate()
                    .min_by_key(|(index, n)| n.cost)
                    .map(|(x, n)| (x, n.clone()))
                    .unwrap();

            if next_node.1.x == self.risks.len() -1 && next_node.1.y == self.risks.len() - 1 {
                // why do we need to subtract the last one?
                return next_node.1.cost as usize;
            }

            // add all neighbors of the next_node
            let neighbors = self.get_neighbors(&next_node.1);
            let n_iter = neighbors.iter()
                .map(|n| Node::new(n.x, n.y, self.risks[n.x][n.y] + next_node.1.cost))
                ;
            // println!("nodeslen: {}", open_nodes.len());

            let to_remove = next_node.0;
            for n in n_iter {
                if visited.get(&(n.x, n.y)).is_none() {
                    visited.insert((n.x, n.y));
                    open_nodes.push(n);
                }
            }
            open_nodes.remove(to_remove);


        }
    }

    fn get_neighbors(&self, node: &Node) -> Vec<Node> {
        let (nx, ny) = (node.x as i32, node.y as i32);
        vec![(-1, 0), ( 0, -1), ( 0, 1), ( 1, 0)]
            .into_iter()
         .map(|(x, y)| (x + nx, y + ny))
         .filter(|(ref x, ref y)| {
             x >= &0 && x < &(self.risks.len() as i32)
                 && y >= &0 && y < &(self.risks.len() as i32)
         })
        .map(|(x, y)| (x as usize, y as usize))
        .map(|(x, y)| Node::new(x, y, self.risks[x][y]))
        .collect()
    }

}

fn parse_input(inp: &str) -> Grid {
    let risks : Vec<Vec<i32>> = inp.lines()
        .map(|x| {
            x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
        })
        .collect();
    risks.into()
}


fn main() {
    let input = std::fs::read_to_string("input/day15")
        .expect("could not read input");
    let grid = parse_input(&input);


    //let grid = parse_input("1163751742
//1381373672
//2136511328
//3694931569
//7463417111
//1319128137
//1359912421
//3125421639
//1293138521
//2311944581");
    // assert_eq!(grid.lowest_path(), 403);
    // println!("{}", grid.lowest_path());

    let grid_p2 = grid.expand();
    let low_score = grid_p2.lowest_path();
    println!("{}", low_score);
    assert_eq!(low_score, 2840);
}


#[test]
fn test_demo() {
    let grid = parse_input("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581");
    assert_eq!(grid.lowest_path(), 40);
}
