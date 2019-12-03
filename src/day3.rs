use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};

use hashbrown::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Direction, ()> {
        Ok(match s {
            "U" => Direction::Up,
            "L" => Direction::Left,
            "R" => Direction::Right,
            "D" => Direction::Down,
            x => panic!("Unknown direction {}", x),
        })
    }
}

#[derive(Debug)]
struct Vector {
    direction: Direction,
    magnitude: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq)]
struct Point {
    x: isize,
    y: isize,
    i: usize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl FromStr for Vector {
    type Err = ();
    fn from_str(s: &str) -> Result<Vector, ()> {
        let (d, mag) = s.split_at(1);
        Ok(Vector {
            direction: d.parse().unwrap(),
            magnitude: mag.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct WirePath(HashSet<Point>);

impl WirePath {
    fn new(wire: &[Vector]) -> Self {
        let mut location = Point::default();
        let mut points = HashSet::with_capacity(200_000);
        for vector in wire {
            let to_add = match vector.direction {
                Direction::Up | Direction::Right => 1,
                Direction::Down | Direction::Left => -1,
            };
            for _ in 1..=vector.magnitude {
                match vector.direction {
                    Direction::Up | Direction::Down => location.y += to_add,
                    Direction::Left | Direction::Right => location.x += to_add,
                };
                location.i += 1;
                points.insert(location.clone());
            }
        }
        Self(points)
    }
}

#[inline(always)]
fn manhattan_distance(p: &Point, q: &Point) -> isize {
    (p.x - q.x).abs() + (p.y - q.y).abs()
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<Vector>> {
    input
        .split('\n')
        .map(|wire| wire.split(',').map(|el| el.parse().unwrap()).collect())
        .collect()
}

const ROOT: Point = Point { x: 0, y: 0, i: 0 };

#[aoc(day3, part1)]
fn part1(input: &[Vec<Vector>]) -> isize {
    let mut iter = input.iter().map(|wire| WirePath::new(wire).0);
    iter.next()
        .map(|set| iter.fold(set, |set1, set2| &set1 & &set2))
        .expect("Expected at least one wire")
        .iter()
        .map(|el| manhattan_distance(&ROOT, &el))
        .min()
        .expect("Wires should intersect at least once!")
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<Vector>]) -> usize {
    let sets: Vec<HashSet<Point>> = input.iter().map(|wire| WirePath::new(wire).0).collect();
    // Unwraps are safe because we have two wires
    (&sets[0] & &sets[1])
        .iter()
        // Unwraps are safe because we've already calculated the intersection
        // so we know all `el`s are in both sets.
        .map(|el| sets[0].get(&el).unwrap().i + sets[1].get(&el).unwrap().i)
        .min()
        .expect("Wires should intersect at least once!")
}

#[cfg(test)]
mod tests {

    use super::{parse_input, part1, part2};

    #[test]
    fn test_part1() {
        let cases = vec![
            ("R8,U5,L5,D3\nU7,R6,D4,L4", 6),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                159,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135,
            ),
        ];
        for case in cases {
            assert_eq!(part1(&parse_input(case.0)), case.1);
        }
    }

    #[test]
    fn test_part2() {
        let cases = vec![
            ("R8,U5,L5,D3\nU7,R6,D4,L4", 30),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                610,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                410,
            ),
        ];
        for case in cases {
            assert_eq!(part2(&parse_input(case.0)), case.1);
        }
    }
}
