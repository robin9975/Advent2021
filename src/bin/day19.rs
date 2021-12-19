
use std::collections::{ HashSet, HashMap };


#[derive(Debug)]
struct Scanner {
    id: usize,
    solved: bool,
    beacons: Vec<(isize, isize, isize)>,
    // the transformation of this scanner, described
    // as a homogeneous transformation matrix
    transformation: Vec<Vec<isize>>,
}


#[derive(Debug)]
struct DMatrix {
    matrix: Vec<Vec<(isize, isize, isize)>>
}

impl DMatrix {
    fn size(&self) -> (usize, usize) {
        (self.matrix[0].len(), self.matrix[0].len())
    }
    fn get(&self, x: usize, y: usize) -> (isize, isize, isize) {
        self.matrix[x][y]
    }

    fn iter(&self) -> DMatrixIterator {
        DMatrixIterator {
            matrix: self,
            row: 0,
            column: 1, // skip the 0,0 coordinate
        }
    }
}


struct DMatrixIterator<'a> {
    matrix: &'a DMatrix,
    row: usize,
    column: usize,
}
impl<'a> Iterator for DMatrixIterator<'a> {
    // row, column, dist
    type Item = (usize, usize, (isize, isize, isize));

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.matrix.size().0 { return None; }

        let (x, y) = (self.row, self.column);
        let item = self.matrix.get(self.row, self.column);

        self.column += 1;
        while self.column <= self.row { self.column += 1; }
        if self.column == self.matrix.size().1 {
            self.column = 0;
            self.row += 1;
        }

        Some((x, y, item))
    }
}



impl Scanner {

    ///
    /// Order of x, y and z and negative / positive doesn't
    /// say anything at this point
    ///
    fn dist_matrix(&self) -> DMatrix {
        let mut matrix = vec![];
        for r in self.beacons.iter() {
            let mut row = vec![];
            for c in self.beacons.iter() {
                row.push((
                    c.0 - r.0,
                    c.1 - r.1,
                    c.2 - r.2
                ));
            }
            matrix.push(row)
        }
        DMatrix { matrix }
    }


    fn with_transformation(&self, transformation: Vec<Vec<isize>>) -> Self {
        let t2 = dot(&self.transformation, &transformation);
        let beacons = self.beacons
            .iter()
            .map(|b| dot(&vec![vec![b.0, b.1, b.2, 1]], &transformation))
            .map(|mut p| p.pop().unwrap())
            .map(|p| (p[0], p[1], p[2]))
            .collect();

        Self {
            id: self.id,
            solved: false,
            beacons,
            transformation: t2,
        }
    }
}

fn dot(left: &Vec<Vec<isize>>, right: &Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    //dbg!(left, right);
    //assert_eq!(left.len(), right[0].len());
    //assert_eq!(left[0].len(), right.len());
    let mut result = vec![];

    for x in 0..left.len() {
        result.push(
            (0..right.len())
            .map(|y| {
                left[x].iter().zip(
                    right.iter().map(|r_row| r_row[y])
                ).map(|(x, y)| x * y).sum()
            })
            .collect::<Vec<isize>>()
        );
    }
    result
}


fn pos_diff(
    l: (usize, usize, usize),
    r: (usize, usize, usize)
) -> (usize, usize, usize) {
    (
        r.0 - l.0,
        r.1 - l.1,
        r.2 - l.2,
    )
}

type BeaconIndex = usize;

///
/// Determine which distance pairs seem to be similar,
/// based on these patterns, we might determine which
/// beaons actually do overlap
///
fn matrix_similar_distances(
    left: DMatrix,
    right: DMatrix
) -> Vec<((BeaconIndex, BeaconIndex), (BeaconIndex, BeaconIndex))> {
    let mut found = vec![];
    for left_item in left.iter() {
        for right_item in right.iter() {
            if dist_similar(left_item.2, right_item.2) {
                found.push((
                    (left_item.0, left_item.1),
                    (right_item.0, right_item.1)
                ));
            }
        }
    }

    found
}


///
/// Based on a set of similar distances, we can determine which
/// beacon pairs actually do correspond with each other.
///
/// This information can be used to determine the actual
/// scanner position
///
fn pairs_from_similar_distances(
    beacon_pairs: &[((BeaconIndex, BeaconIndex), (BeaconIndex, BeaconIndex))]
) -> Vec<(BeaconIndex, BeaconIndex)> {
    let mut matches : Vec<(BeaconIndex, BeaconIndex)> = vec![];
    // which beacons on the left are we trying to match
    // with a specific one on the right?
    let mut to_match = beacon_pairs
        .iter()
        .flat_map(|x| vec![x.0.0, x.0.1])
        .collect::<Vec<BeaconIndex>>();
    to_match.sort();
    to_match.dedup();

    for index in to_match {
        // only the right one
        let mut pit = beacon_pairs.iter()
            .filter(|x| x.0.0 == index || x.0.1 == index)
            .map(|x| x.1);

        let first = pit.next().unwrap();
        let mut options = vec![first.0, first.1].into_iter().collect::<HashSet<_>>();

        for p in pit {
            let opt2 = vec![p.0, p.1]
                .into_iter()
                .collect::<HashSet<_>>();

            options = options.intersection(&opt2).cloned().collect();
            if options.len() <= 1 { break; }
        }

        if options.len() == 1 {
            matches.push((index, options.drain().next().unwrap()));
        }
    }

    matches
}


fn determine_scanner_orientation(
    scanner0: &Scanner,
    scanner1: &Scanner,
    similar_distances: &Vec<((BeaconIndex, BeaconIndex), (BeaconIndex, BeaconIndex))>
) -> Vec<Vec<isize>> {
    let ld = scanner0.dist_matrix();
    let rd = scanner1.dist_matrix();

    let mut ds = similar_distances.iter();
    let (left, right) = ds.next().unwrap();
    let (lx, ly, lz) = ld.get(left.0, left.1);
    let (rx, ry, rz) = rd.get(right.0, right.1);

    let x =
        if lx == rx { vec![1, 0, 0, 0] }
        else if lx == -rx { vec![-1, 0, 0, 0] }
        else if lx == ry { vec![0, 1, 0, 0] }
        else if lx == -ry { vec![0, -1, 0, 0] }
        else if lx == rz { vec![0, 0, 1, 0] }
        else if lx == -rz { vec![0, 0, -1, 0] }
        else { unreachable!() };

    let y =
        if ly == rx { vec![1, 0, 0, 0] }
        else if ly == -rx { vec![-1, 0, 0, 0] }
        else if ly == ry { vec![0, 1, 0, 0] }
        else if ly == -ry { vec![0, -1, 0, 0] }
        else if ly == rz { vec![0, 0, 1, 0] }
        else if ly == -rz { vec![0, 0, -1, 0] }
        else { unreachable!() };

    let z =
        if lz == rx { vec![1, 0, 0, 0] }
        else if lz == -rx { vec![-1, 0, 0, 0] }
        else if lz == ry { vec![0, 1, 0, 0] }
        else if lz == -ry { vec![0, -1, 0, 0] }
        else if lz == rz { vec![0, 0, 1, 0] }
        else if lz == -rz { vec![0, 0, -1, 0] }
        else { unreachable!() };

    let transformation = vec![
        x, y, z, vec![0, 0, 0, 1]
    ];

    // TODO: Assert that the transformation works for
    // all other distances?

    transformation
}


fn determine_scanner_position(
    s0: &Scanner,
    s1: &Scanner,
    pair: (BeaconIndex, BeaconIndex)
) -> Vec<Vec<isize>> {
    let b0 = s0.beacons[pair.0];
    let b1 = s1.beacons[pair.1];

    vec![
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 1, 0],
        vec![b0.0 - b1.0, b0.1 - b1.1, b0.2 - b1.2, 1]
    ]
}


///
/// A distance is deemed similar if the vales
/// of the distances in any of the axes correspond.
///
fn dist_similar(
    left: (isize, isize, isize),
    right: (isize, isize, isize)
) -> bool {
    let mut l = vec![left.0.abs(), left.1.abs(), left.2.abs()];
    l.sort();
    let mut r = vec![right.0.abs(), right.1.abs(), right.2.abs()];
    r.sort();

    l == r
}



///
/// Parse a single scanner instance from a text file
///
fn parse_scanner(input: &str) -> Scanner {
    let mut lines = input.lines();
    lines.next();

    let beacons = lines
        .map(|l| {
            let mut i2 = l.split_terminator(",")
             .map(|x| -> isize { x.parse().unwrap() });
            (
                i2.next().unwrap(),
                i2.next().unwrap(),
                i2.next().unwrap()
            )
        })
        .collect();

    Scanner {
        id: 0,
        solved: false,
        beacons,
        transformation: vec![
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 1],
    ] }
}



fn solve_scanners(mut scanners: Vec<Scanner>) -> Vec<Scanner> {
    scanners[0].solved = true;

    loop {
        let unsolved_iter = scanners.iter().enumerate().filter(|(_, x)| !x.solved);

        let mut set_solved : HashMap<usize, Scanner> = HashMap::new();

        for (index, s) in unsolved_iter {
            for (key, solved) in scanners.iter().enumerate().filter(|(_, x)| x.solved) {
                let distances = matrix_similar_distances(
                    solved.dist_matrix(),
                    s.dist_matrix()
                );
                let pairs = pairs_from_similar_distances(&distances);
                if pairs.len() < 12 {
                    continue;
                }
                println!("Solved {} based on {}", index, key);

                let orientation = determine_scanner_orientation(&solved, &s, &distances);
                let mut new_scanner = s.with_transformation(orientation);

                let position = determine_scanner_position(&solved, &new_scanner, pairs[0]);
                new_scanner = new_scanner.with_transformation(position);
                // new_scanner = new_scanner.with_transformation(solved.transformation.clone());
                new_scanner.solved = true;
                set_solved.insert(index, new_scanner);
            }
        }

        for (index, scanner) in set_solved.into_iter() {
            scanners[index] = scanner;
        }


        let solved_count = scanners.iter()
            .filter(|x| x.solved)
            .count();
        if solved_count == scanners.len() { break; }
    }

    scanners
}

fn get_unique_beacon_count(scanners: &[Scanner]) -> usize {
    let set : HashSet<(isize, isize, isize)> = scanners.iter()
        .flat_map(|x| x.beacons.clone())
        .collect();

    let mut v = set.iter().cloned().collect::<Vec<_>>();
    v.sort();
    dbg!(v);
    set.len()
}

fn main() {
    let input = std::fs::read_to_string("input/day19_test").unwrap();
    let scanners = input.split_terminator("\n\n")
        .map(|inp| parse_scanner(inp))
        .collect::<Vec<Scanner>>();

    let solved_scanners = solve_scanners(scanners);
    dbg!(solved_scanners.iter().map(|x| &x.transformation).collect::<Vec<_>>());
    // dbg!(get_unique_beacon_count(&solved_scanners));
}



#[test]
fn test_parse_scanner() {
    let input = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401";

    parse_scanner(input).dist_matrix();
}



#[test]
fn test_similar_distances() {
    let s0 = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401";


    let s1 = "--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390";

    let s0 = parse_scanner(s0);
    let mut s1 = parse_scanner(s1);
    let distances = matrix_similar_distances(
        s0.dist_matrix(),
        s1.dist_matrix()
    );

    let pairs = pairs_from_similar_distances(&distances);
    let orientation = determine_scanner_orientation(&s0, &s1, &distances);
    s1 = s1.with_transformation(orientation);

    let position = determine_scanner_position(&s0, &s1, pairs[0]);
    s1 = s1.with_transformation(position);

    for (left, right) in pairs {
        assert_eq!(s0.beacons[left], s1.beacons[right]);
    }

    // dbg!(orientation);
}
