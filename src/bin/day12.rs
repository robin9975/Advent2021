

#[derive(Clone, Debug)]
struct Cave {
    name: String,
    is_small: bool
}


impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        Self {
            name: input.to_string(),
            is_small: input.chars().next().unwrap().is_lowercase(),
        }
    }
}

struct CaveSystem {
    tunnels: Vec<(Cave, Cave)>,
    small_cave_visited: Option<String>,
}

impl CaveSystem {

    fn get_targets(&self, current: &str) -> Vec<Cave> {
        self.tunnels
            .iter()
            .filter_map(|(from, to)| {
                if &from.name == current { return Some(to); }
                if &to.name == current { return Some(from); }
                None
            })
            .cloned()
            .collect()
    }

    fn get_with_subcave_visited(&self, cave_name: &str) -> Self {
        Self {
            tunnels: tunnels.clone(),
            small_cave_visited: Some(cave_name.to_string()),
        }
    }

    fn get_subcaves_without(&self, cave_name: &str) -> Self {
        Self {
            tunnels: self.tunnels
                .iter()
                .filter(|(from, to)| {
                    from.name != cave_name && to.name != cave_name
                })
                .cloned()
                .collect()
        }
    }

    fn get_all_paths(&self) -> Vec<Vec<Cave>> {
        self.get_paths_from("start")
    }

    fn get_paths_from(&self, label: &str) -> Vec<Vec<Cave>> {
        let c : Cave = label.into();

        let targets = self.get_targets(label);
        let mut paths = vec![];

        let subcaves = self.get_subcaves_without(label);
        let newcaves = if c.is_small { &subcaves } else { self };

        for t in targets.iter() {
            if t.name == "end"  {
                paths.push(vec![label.into(), "end".into()]);
            } else {

                if self.small_cave_visited.is_none() && c.is_small {
                    let extracaves = self.get_with_subcave_visited(&c.name);
                    for p in extracaves.get_paths_from(&t.name).into_iter() {
                        let mut yy = vec![label.into()];
                        yy.extend(p);
                        paths.push(yy);
                    }
                }

                for p in newcaves.get_paths_from(&t.name).into_iter() {
                    let mut yy = vec![label.into()];
                    yy.extend(p);
                    paths.push(yy);
                }
            }
        }

        paths
    }
}


fn parse_input(input: String) -> CaveSystem {
    let tunnels: Vec<(Cave, Cave)> = input.lines()
        .map(|x: &str| {
            let mut y = x.split_terminator("-");
            (
                y.next().unwrap().into(),
                y.next().unwrap().into()
            )
        })
        .collect();
    CaveSystem { tunnels }
}


fn main() {
    let input = std::fs::read_to_string("input/day12").unwrap();
    let caves = parse_input(input);
    println!("{}", caves.get_all_paths().len());
}

#[test]
fn test_first_demo() {
    let caves = parse_input("start-A
start-b
A-c
A-b
b-d
A-end
b-end".to_string()
    );
    assert_eq!(caves.get_all_paths().len(), 10);
}

#[test]
fn test_2nd_demo() {
    let caves = parse_input("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc".to_string()
    );
    assert_eq!(caves.get_all_paths().len(), 19);
}

