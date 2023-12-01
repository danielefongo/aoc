use std::collections::{HashMap, HashSet, VecDeque};

use utils::{extract, extract_one, lines, read_input};

pub fn run() {
    let blueprints: Vec<Blueprint> = lines(read_input!())
        .into_iter()
        .map(Blueprint::from)
        .collect();

    println!(
        "Part1: {}",
        blueprints
            .iter()
            .map(|it| it.idx * execute_blueprint(it.clone(), 24))
            .sum::<i32>()
    );

    println!(
        "Part2: {}",
        blueprints
            .iter()
            .take(3)
            .map(|it| execute_blueprint(it.clone(), 32))
            .product::<i32>()
    );
}

fn execute_blueprint(blueprint: Blueprint, end: i32) -> i32 {
    let maxes: Vec<i32> = (0..4)
        .map(|ore| match ore {
            0 => i32::MAX,
            _ => blueprint.costs.iter().map(|it| it[ore]).max().unwrap(),
        })
        .collect();

    let mut visited: HashSet<Blueprint> = HashSet::new();
    let mut queue: VecDeque<Blueprint> = VecDeque::new();
    let mut best_ores_by_iteration: HashMap<i32, i32> = HashMap::new();
    let mut best = blueprint.clone();
    let target = 0;

    queue.push_front(blueprint);

    while let Some(blueprint) = queue.pop_front() {
        if visited.contains(&blueprint) {
            continue;
        }
        visited.insert(blueprint.clone());

        let mut best_blueprint = blueprint.clone();
        best_blueprint.make_robot_free(target);
        while best_blueprint.iterations < end {
            best_blueprint.produce(target);
        }
        if best_blueprint.ores[target] <= best.ores[target] {
            continue;
        }

        let mut ended_blueprint = blueprint.clone();
        ended_blueprint.farm(1);

        if &ended_blueprint.ores[target]
            < best_ores_by_iteration
                .get(&ended_blueprint.iterations)
                .unwrap_or(&0)
        {
            continue;
        }
        best_ores_by_iteration.insert(ended_blueprint.iterations, ended_blueprint.ores[target]);

        ended_blueprint.farm(end - blueprint.iterations - 1);
        if ended_blueprint.ores[target] > best.ores[target] {
            best = ended_blueprint.clone();
        }

        for ore in 0..4 {
            if blueprint.robots[ore] == maxes[ore] {
                continue;
            }
            let needed_rounds = blueprint.rounds_to_produce(ore);

            if blueprint.iterations + needed_rounds > end - 1 {
                continue;
            }

            let mut new_blueprint = blueprint.clone();

            new_blueprint.farm(needed_rounds - 1);
            new_blueprint.produce(ore);

            queue.push_front(new_blueprint);
        }
    }

    best.ores[target]
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Blueprint {
    idx: i32,
    iterations: i32,
    robots: Vec<i32>,
    costs: Vec<Vec<i32>>,
    ores: Vec<i32>,
}
impl Blueprint {
    fn empty(idx: i32) -> Self {
        Self::new(
            idx,
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        )
    }
    fn new(idx: i32, costs: Vec<Vec<i32>>, robots: Vec<i32>, ores: Vec<i32>) -> Self {
        Self {
            idx,
            robots,
            costs,
            ores,
            iterations: 0,
        }
    }
    fn farm(&mut self, rounds: i32) {
        self.ores = (0..4)
            .map(|idx| self.ores[idx] + self.robots[idx] * rounds)
            .collect();

        self.iterations += rounds;
    }
    fn make_robot_free(&mut self, robot: usize) {
        self.costs[robot] = vec![0, 0, 0, 0];
    }
    fn produce(&mut self, robot: usize) -> bool {
        let costs = self.costs[robot].clone();
        if (0..4).any(|idx| costs[idx] > self.ores[idx]) {
            return false;
        }

        self.ores = (0..4)
            .map(|idx| self.ores[idx] + self.robots[idx] - costs[idx])
            .collect();

        self.robots[robot] += 1;

        self.iterations += 1;

        true
    }
    fn rounds_to_produce(&self, robot: usize) -> i32 {
        let costs = self.costs[robot].clone();
        (robot..4)
            .map(|ore| match (costs[ore], self.ores[ore], self.robots[ore]) {
                (c, _, 0) if c > 0 => i32::MAX / 2,
                (0, _, _) => 1,
                (c, o, _r) if o >= c => 1,
                (c, o, r) => ((c - o) as f32 / r as f32).ceil() as i32 + 1,
            })
            .max()
            .unwrap()
    }
}
impl From<String> for Blueprint {
    fn from(input: String) -> Self {
        let robots = vec![0, 0, 0, 1];
        let ores = vec![0, 0, 0, 0];
        let prefix = extract_one(&input, "Blueprint \\d+");
        let numbers: Vec<i32> = extract(&input.replace(&prefix, ""), "\\d+")
            .into_iter()
            .map(|it| it.parse().unwrap())
            .collect();
        let idx = prefix.replace("Blueprint ", "").parse().unwrap();
        let ore_costs = vec![0, 0, 0, numbers[0]];
        let clay_costs = vec![0, 0, 0, numbers[1]];
        let obsidian_costs = vec![0, 0, numbers[3], numbers[2]];
        let geode_costs = vec![0, numbers[5], 0, numbers[4]];

        Self::new(
            idx,
            vec![geode_costs, obsidian_costs, clay_costs, ore_costs],
            robots,
            ores,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::Blueprint;

    #[test]
    fn parse_blueprint() {
        let blueprint = Blueprint::from("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".to_string());

        assert_eq!(
            blueprint.costs,
            vec![
                vec![0, 7, 0, 2],
                vec![0, 0, 14, 3],
                vec![0, 0, 0, 2],
                vec![0, 0, 0, 4],
            ]
        );

        assert_eq!(blueprint.robots, vec![0, 0, 0, 1]);
        assert_eq!(blueprint.ores, vec![0, 0, 0, 0]);
    }

    #[test]
    fn farm_ores() {
        let mut blueprint = Blueprint::empty(1);
        blueprint.robots = vec![2, 0, 1, 0];
        blueprint.ores = vec![0, 0, 1, 0];

        blueprint.farm(1);

        assert_eq!(blueprint.ores, vec![2, 0, 2, 0]);
    }

    #[test]
    fn produce_robots() {
        let mut blueprint = Blueprint::empty(1);
        blueprint.ores = vec![0, 1, 1, 4];
        blueprint.costs = vec![
            vec![0, 0, 0, 3],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 5],
            vec![0, 0, 0, 1],
        ];

        assert_eq!(blueprint.produce(0), true);
        assert_eq!(blueprint.produce(1), true);
        assert_eq!(blueprint.produce(2), false);
        assert_eq!(blueprint.produce(3), true);
        assert_eq!(blueprint.robots, vec![1, 1, 0, 1]);
    }

    #[test]
    fn rounds_to_produce() {
        let mut blueprint = Blueprint::empty(1);
        blueprint.robots = vec![0, 0, 2, 1];
        blueprint.ores = vec![0, 0, 1, 0];
        blueprint.costs = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 5, 1],
            vec![0, 0, 0, 1],
            vec![0, 0, 1, 0],
        ];

        assert_eq!(blueprint.rounds_to_produce(0), i32::MAX / 2);
        assert_eq!(blueprint.rounds_to_produce(1), 3);
        assert_eq!(blueprint.rounds_to_produce(2), 2);
        assert_eq!(blueprint.rounds_to_produce(3), 1);
    }
}
