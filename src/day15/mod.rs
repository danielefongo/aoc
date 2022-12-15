use std::collections::HashSet;

use crate::utils::{extract, lines, read_input};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);
impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as i32
    }
}
impl From<String> for Point {
    fn from(input: String) -> Self {
        let data = extract(&input, "-?\\d+");
        Point(data[0].parse().unwrap(), data[1].parse().unwrap())
    }
}

#[derive(Clone, Debug)]
struct SensorAndBeacon {
    sensor: Point,
    beacon: Point,
    range: i32,
}
impl SensorAndBeacon {
    fn new(sensor: Point, beacon: Point) -> Self {
        Self {
            sensor,
            beacon,
            range: sensor.distance(&beacon) as i32,
        }
    }
    fn reaches(&self, point: &Point) -> bool {
        self.sensor.distance(point) as i32 <= self.range
    }
    fn near(&self, other: &SensorAndBeacon) -> bool {
        self.range + other.range == self.sensor.distance(&other.sensor) - 2
    }
    fn border(&self) -> HashSet<Point> {
        let mean_x = self.sensor.0;
        let mean_y = self.sensor.1;
        let min_x = mean_x - self.range;
        let min_y = mean_y - self.range;
        let max_x = mean_x + self.range;
        let max_y = mean_y + self.range;

        [
            (min_x..=mean_x).zip(mean_y..=max_y),
            (min_x..=mean_x).zip(mean_y..=min_y),
            (mean_x..=max_x).zip(max_y..=mean_y),
            (mean_x..=max_x).zip(min_y..=mean_y),
        ]
        .into_iter()
        .flat_map(|iter| iter.map(|(x, y)| Point(x, y)))
        .collect()
    }
    fn extended(&self) -> SensorAndBeacon {
        Self::new(
            self.sensor,
            Point(self.sensor.0 + self.range + 1, self.sensor.1),
        )
    }
    fn candidates(&self, other: &SensorAndBeacon) -> HashSet<Point> {
        if !self.near(other) {
            return HashSet::new();
        }

        self.extended()
            .border()
            .into_iter()
            .filter(|&it| {
                it.distance(&other.sensor) == other.range + 1
                    && it.0 != other.sensor.0
                    && it.1 != other.sensor.1
                    && it.0 != self.sensor.0
                    && it.1 != self.sensor.1
            })
            .collect()
    }
}
impl From<String> for SensorAndBeacon {
    fn from(input: String) -> Self {
        let data = extract(&input, "x=-?\\d+, y=-?\\d+");
        Self::new(data[0].to_string().into(), data[1].to_string().into())
    }
}

pub fn run() {
    let sensors_and_beacons: Vec<SensorAndBeacon> = lines(read_input(15))
        .into_iter()
        .map(SensorAndBeacon::from)
        .collect();
    part1(&sensors_and_beacons);
    part2(&sensors_and_beacons);
}

fn part1(sensors_and_beacons: &Vec<SensorAndBeacon>) {
    let y = 2000000;
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    sensors_and_beacons
        .iter()
        .flat_map(|sb| vec![sb.sensor.0 - sb.range, sb.sensor.0 + sb.range])
        .for_each(|b| {
            min_x = b.min(min_x);
            max_x = b.max(max_x);
        });

    let count = (min_x..max_x)
        .filter(|&x| {
            let candidate = Point(x, y);
            sensors_and_beacons
                .iter()
                .any(|sb| sb.reaches(&candidate) && candidate != sb.beacon)
        })
        .count();

    println!("Part1: {}", count);
}

fn part2(sensors_and_beacons: &Vec<SensorAndBeacon>) {
    let point = sensors_and_beacons
        .iter()
        .cloned()
        .flat_map(|first| {
            sensors_and_beacons
                .iter()
                .skip(1)
                .flat_map(|second| first.candidates(&second))
                .collect::<Vec<Point>>()
        })
        .find(|&it| sensors_and_beacons.iter().all(|sb| !sb.reaches(&it)))
        .unwrap();

    println!("Part2: {}", (point.0 as u128) * 4000000 + (point.1 as u128));
}

#[cfg(test)]
mod test {
    mod point {
        use crate::day15::Point;

        #[test]
        fn distance() {
            assert_eq!(Point(0, 0).distance(&Point(0, 5)), 5);
            assert_eq!(Point(0, 0).distance(&Point(5, 0)), 5);
            assert_eq!(Point(1, 1).distance(&Point(2, 3)), 3);
            assert_eq!(Point(1, 1).distance(&Point(-1, 3)), 4);
        }
    }

    mod sensor_and_beacon {
        use std::collections::HashSet;

        use crate::day15::{Point, SensorAndBeacon};

        #[test]
        fn reaches() {
            let sensor = SensorAndBeacon::new(Point(0, 0), Point(5, 0));
            assert_eq!(sensor.reaches(&Point(0, 0)), true);
            assert_eq!(sensor.reaches(&Point(-5, 0)), true);
            assert_eq!(sensor.reaches(&Point(5, 0)), true);
            assert_eq!(sensor.reaches(&Point(0, -5)), true);
            assert_eq!(sensor.reaches(&Point(0, 5)), true);

            assert_eq!(sensor.reaches(&Point(2, 3)), true);
            assert_eq!(sensor.reaches(&Point(1, 4)), true);

            assert_eq!(sensor.reaches(&Point(2, 4)), false);
            assert_eq!(sensor.reaches(&Point(-1, 5)), false);
        }

        #[test]
        fn close() {
            let sensor1 = SensorAndBeacon::new(Point(0, 0), Point(0, 2));
            let sensor2 = SensorAndBeacon::new(Point(2, -3), Point(2, -4));
            let sensor3 = SensorAndBeacon::new(Point(4, -1), Point(6, -1));
            let sensor4 = SensorAndBeacon::new(Point(2, 2), Point(2, 3));
            let sensor5 = SensorAndBeacon::new(Point(4, 4), Point(6, 4));

            assert_eq!(sensor1.near(&sensor2), true);
            assert_eq!(sensor1.near(&sensor3), false);
            assert_eq!(sensor1.near(&sensor4), false);
            assert_eq!(sensor1.near(&sensor5), false);

            assert_eq!(sensor2.near(&sensor3), false);
            assert_eq!(sensor2.near(&sensor4), false);
            assert_eq!(sensor2.near(&sensor5), false);

            assert_eq!(sensor3.near(&sensor4), true);
            assert_eq!(sensor3.near(&sensor5), false);

            assert_eq!(sensor4.near(&sensor5), false);
        }

        #[test]
        fn border() {
            let expected: HashSet<Point> =
                vec![Point(0, 1), Point(0, -1), Point(1, 0), Point(-1, 0)]
                    .into_iter()
                    .collect();

            let sensor = SensorAndBeacon::new(Point(0, 0), Point(0, 1));
            assert_eq!(sensor.border(), expected);
        }

        #[test]
        fn candidates() {
            let sensor1 = SensorAndBeacon::new(Point(0, 0), Point(0, 2));
            let sensor2 = SensorAndBeacon::new(Point(2, -3), Point(2, -4));
            let sensor3 = SensorAndBeacon::new(Point(4, -1), Point(6, -1));
            let sensor4 = SensorAndBeacon::new(Point(2, 2), Point(2, 3));
            let sensor5 = SensorAndBeacon::new(Point(3, -4), Point(6, -4));

            assert_eq!(
                sensor1.candidates(&sensor2),
                vec![Point(1, -2)].into_iter().collect()
            );
            assert_eq!(
                sensor3.candidates(&sensor4),
                vec![Point(3, 1)].into_iter().collect()
            );
            assert_eq!(
                sensor1.candidates(&sensor5),
                vec![Point(1, -2), Point(2, -1)].into_iter().collect()
            );
        }
    }
}
