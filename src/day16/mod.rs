use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::utils::{extract_one, lines, read_input};

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
    nodes: HashSet<String>,
    distances: HashMap<String, HashMap<String, usize>>,
    values: HashMap<String, usize>,
}
impl Solver {
    fn new(
        nodes: Vec<String>,
        distances: HashMap<String, HashMap<String, usize>>,
        values: HashMap<String, usize>,
    ) -> Self {
        Self {
            nodes: nodes
                .into_iter()
                .filter(|it| values.get(it).unwrap() > &0)
                .collect::<HashSet<_>>(),
            distances,
            values,
        }
    }
    fn solve(&self) -> usize {
        let mut paths = HashMap::new();
        self.try_permutations(&mut paths, Path::new(), "AA".to_string(), 30, 0);
        paths.iter().map(|(_, score)| score).max().unwrap().clone()
    }
    fn try_permutations(
        &self,
        paths: &mut HashMap<Path, usize>,
        visited_path: Path,
        actual_node: String,
        time_remaining: usize,
        score: usize,
    ) {
        for target_node in self.nodes.difference(&visited_path.as_hashset()) {
            let distance = get(&self.distances, &actual_node, &target_node).clone();

            if time_remaining >= distance + 1 {
                let target_flow = self.values.get(target_node).unwrap().clone();

                let time_remaining = time_remaining - distance - 1;
                let score = score + time_remaining * target_flow;

                let mut path = visited_path.clone();
                path.push(target_node.clone());

                let visited_path_score = paths.get(&visited_path).unwrap_or(&0);
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
        self.path
            .iter()
            .map(|it| it.clone())
            .collect::<Vec<String>>()
            .join("")
            .hash(state)
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
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    let mut distances: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut values: HashMap<String, usize> = HashMap::new();
    let mut nodes: HashSet<String> = HashSet::new();

    lines(read_input(16))
        .into_iter()
        .map(parse_line)
        .for_each(|(from, rate, tos)| {
            let hashset = tos.clone().into_iter().collect::<HashSet<String>>();
            *connections.entry(from.clone()).or_insert(HashSet::new()) = hashset;

            nodes.insert(from.clone());
            values.insert(from.clone(), rate);
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

    let solver = Solver::new(nodes.into_iter().collect::<Vec<_>>(), distances, values);
    println!("Part1: {}", solver.solve());
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
