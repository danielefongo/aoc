use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use utils::{extract_one, lines, read_input};

fn get<'a, T: Hash + Eq, Z>(hashmap: &'a HashMap<T, HashMap<T, Z>>, f: &T, t: &T) -> &'a Z {
    hashmap.get(f).unwrap().get(t).unwrap()
}
fn get_mut<'a, T: Hash + Eq + Clone, Z: Default>(
    hashmap: &'a mut HashMap<T, HashMap<T, Z>>,
    f: &T,
    t: &T,
) -> &'a mut Z {
    let inner = hashmap.entry(f.clone()).or_insert(HashMap::new());
    inner.entry(t.clone()).or_insert(Z::default())
}

struct Solver {
    mapping: HashMap<String, u64>,
    distances: HashMap<u64, HashMap<u64, usize>>,
    values: HashMap<u64, usize>,
}
impl Solver {
    fn new(
        mapping: HashMap<String, u64>,
        distances: HashMap<u64, HashMap<u64, usize>>,
        values: HashMap<u64, usize>,
    ) -> Self {
        Self {
            mapping,
            distances,
            values,
        }
    }
    fn solve1(&self) -> usize {
        let mut paths = HashMap::new();
        let input = self.mapping.get("AA").unwrap().clone();
        self.try_permutations(&mut paths, 0, input, 30, 0);
        paths.iter().map(|(_, score)| score).max().unwrap().clone()
    }
    fn solve2(&self) -> usize {
        let mut paths = HashMap::new();
        let input = self.mapping.get("AA").unwrap().clone();
        self.try_permutations(&mut paths, 0, input, 26, 0);

        let mut candidate_paths: Vec<(u64, usize)> =
            paths.into_iter().filter(|(p, _)| p > &0).collect();
        candidate_paths.sort();
        candidate_paths.reverse();

        let mut top = 0;

        for (i, f) in candidate_paths.iter().enumerate() {
            for t in candidate_paths.iter().skip(i) {
                if f.0 & t.0 == 0 {
                    top = top.max(f.1 + t.1);
                }
            }
        }

        top
    }
    fn try_permutations(
        &self,
        paths: &mut HashMap<u64, usize>,
        visited_path: u64,
        actual_node: u64,
        time_remaining: usize,
        score: usize,
    ) {
        for target_node in self.mapping.values() {
            if visited_path & target_node > 0 {
                continue;
            }

            let distance = get(&self.distances, &actual_node, &target_node).clone();

            if time_remaining >= distance + 1 {
                let target_flow = self.values.get(target_node).unwrap().clone();

                if target_flow == 0 {
                    continue;
                }

                let time_remaining = time_remaining - distance - 1;
                let score = score + time_remaining * target_flow;

                let path = visited_path | target_node;

                let visited_path_score = paths.get(&path).unwrap_or(&0);
                paths.insert(path.clone(), score.max(visited_path_score.clone()));

                self.try_permutations(paths, path, target_node.clone(), time_remaining, score)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Path {
    path: Vec<String>,
}
impl Path {
    fn new() -> Self {
        Self { path: Vec::new() }
    }
    fn as_hashset(&self) -> HashSet<String> {
        self.path
            .iter()
            .map(|it| it.clone())
            .collect::<HashSet<String>>()
    }
}
impl Hash for Path {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.join("").hash(state)
    }
}
impl Deref for Path {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.path
    }
}

pub fn run() {
    let mut mapping: HashMap<String, u64> = HashMap::new();
    let mut connections: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut distances: HashMap<u64, HashMap<u64, usize>> = HashMap::new();
    let mut values: HashMap<u64, usize> = HashMap::new();
    let mut nodes: HashSet<u64> = HashSet::new();

    lines(read_input!())
        .into_iter()
        .enumerate()
        .for_each(|(idx, input)| {
            let from = extract_one(&input, "Valve \\w+").replace("Valve ", "");
            let value = (2 as u64).pow(idx as u32);
            mapping.insert(from.clone(), value);
        });

    lines(read_input!())
        .into_iter()
        .map(parse_line)
        .enumerate()
        .for_each(|(idx, (from, rate, tos))| {
            let hashset = tos
                .clone()
                .into_iter()
                .map(|it| mapping.get(&it).unwrap().clone())
                .collect::<HashSet<u64>>();
            let from_value = mapping.get(&from).unwrap().clone();

            *connections.entry(from_value).or_insert(HashSet::new()) = hashset;
            nodes.insert(from_value);
            values.insert(from_value, rate);
        });

    connections.keys().into_iter().for_each(|f| {
        connections.keys().into_iter().for_each(|t| {
            let distance = if connections.get(f).unwrap().contains(t) {
                1
            } else {
                usize::MAX / 2
            };

            *get_mut(&mut distances, f, t) = distance;
        });
    });

    for k in connections.keys() {
        for i in connections.keys() {
            for j in connections.keys() {
                let first = get(&distances, i, k).clone();
                let second = get(&distances, k, j).clone();
                let reference = get_mut(&mut distances, i, j);
                *reference = reference.clone().min(first + second);
            }
        }
    }

    let solver = Solver::new(mapping, distances, values);
    println!("Part1: {}", solver.solve1());
    println!("Part2: {}", solver.solve2());
}

fn parse_line(line: String) -> (String, usize, Vec<String>) {
    let rate = extract_one(&line, "rate=\\d+")
        .replace("rate=", "")
        .parse()
        .unwrap();
    let from = extract_one(&line, "Valve \\w+").replace("Valve ", "");
    let tos = extract_one(&line, "valves? .*$")
        .replace("valves ", "")
        .replace("valve ", "")
        .split(", ")
        .map(|it| it.to_string())
        .collect();
    (from, rate, tos)
}
