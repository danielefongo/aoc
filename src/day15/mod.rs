use crate::utils::{extract, lines, read_input};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point(i32, i32);
impl Point {
    fn distance(&self, other: &Point) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }
}
impl From<String> for Point {
    fn from(input: String) -> Self {
        let data = extract(&input, "-?\\d+");
        Point(data[0].parse().unwrap(), data[1].parse().unwrap())
    }
}

#[derive(Debug)]
struct SensorAndBeacon {
    sensor: Point,
    beacon: Point,
}
impl SensorAndBeacon {
    fn new(sensor: Point, beacon: Point) -> Self {
        Self { sensor, beacon }
    }
    fn range(&self) -> i32 {
        self.sensor.distance(&self.beacon) as i32
    }
    fn reaches(&self, point: &Point) -> bool {
        self.sensor.distance(point) as i32 <= self.range()
    }
    fn reaches_y(&self, y: i32) -> bool {
        self.reaches(&Point(self.sensor.0, y))
    }
}
impl From<String> for SensorAndBeacon {
    fn from(input: String) -> Self {
        let data = extract(&input, "x=-?\\d+, y=-?\\d+");
        Self {
            sensor: data[0].to_string().into(),
            beacon: data[1].to_string().into(),
        }
    }
}

pub fn run() {
    let y = 2000000;
    let sensors_and_beacons: Vec<SensorAndBeacon> = lines(read_input(15))
        .into_iter()
        .map(SensorAndBeacon::from)
        .collect();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    sensors_and_beacons
        .iter()
        .map(|sb| vec![sb.sensor.0 - sb.range(), sb.sensor.0 + sb.range()])
        .flatten()
        .for_each(|b| {
            min_x = if b < min_x { b } else { min_x };
            max_x = if b > max_x { b } else { max_x };
        });

    let filtered: Vec<&SensorAndBeacon> = sensors_and_beacons
        .iter()
        .filter(|it| it.reaches_y(y))
        .collect();

    let count = (min_x..max_x)
        .filter(|&x| {
            filtered.iter().any(|sb| {
                let candidate = Point(x, y);
                sb.reaches(&candidate) && candidate != sb.beacon
            })
        })
        .count();

    println!("Part1: {}", count);
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
        fn reaches_x() {
            let sensor = SensorAndBeacon::new(Point(0, 0), Point(5, 0));
            assert_eq!(sensor.reaches_y(-5), true);
            assert_eq!(sensor.reaches_y(0), true);
            assert_eq!(sensor.reaches_y(5), true);
            assert_eq!(sensor.reaches_y(6), false);
        }
    }
}
