use std::collections::HashMap;
#[cfg(test)]
use std::collections::HashSet;

use pathfinding::prelude::dijkstra;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));

    Ok(())
}

struct Map {
    points: HashMap<Pos, u32>,
}

fn part1(map: &Map) -> u32 {
    let end = map.max();

    let path = dijkstra(
        &Pos { x: 1, y: 1 },
        |pos| {
            pos.adjacents()
                .filter_map(|adj| match map.points.get(&adj) {
                    Some(&cost) => Some((adj, cost)),
                    None => None,
                })
        },
        |pos| pos == &end,
    );

    path.unwrap().1
}

fn part2(map: &Map) -> u32 {
    let max = map.max();
    let end = Pos {
        x: max.x * 5,
        y: max.y * 5,
    };

    let cost_five = |pos: &Pos| {
        if pos.x == 0 || pos.y == 0 {
            return None;
        }
        if pos.x > end.x || pos.y > end.y {
            return None;
        }

        let (source, remainder) = pos.div();

        let &source_cost = map.points.get(&source).unwrap();

        let cost = wrap(source_cost + (remainder.x + remainder.y) as u32);

        Some(cost)
    };

    assert!(cost_five(&Pos { x: 1, y: 1 }).is_some());
    assert!(cost_five(&end).is_some());
    assert!(cost_five(&Pos {
        x: end.x + 1,
        y: end.y
    })
    .is_none());

    let path = dijkstra(
        &Pos { x: 1, y: 1 },
        |cur| {
            cur.adjacents().filter_map(|pos| {
                let cost = cost_five(&pos)?;

                Some((pos, cost))
            })
        },
        |cur| cur == &end,
    );

    let path = path.expect("no path to end?");

    #[cfg(test)]
    {
        let pathpoints: HashSet<_> = path.0.into_iter().collect();

        for y in 1..=end.y {
            for x in 1..=end.x {
                let pos = Pos { x, y };

                if pathpoints.contains(&pos) {
                    print!("\x1b[1;32m");
                }

                print!("{}", cost_five(&pos).unwrap());

                if pathpoints.contains(&pos) {
                    print!("\x1b[0;0m");
                }
            }
            println!();
        }
    }

    path.1
}

fn wrap(cost: u32) -> u32 {
    (cost - 1) % 9 + 1
}

fn div(mut i: i32) -> (i32, i32) {
    let mut n = 0;
    while i > 10 {
        i -= 10;
        n += 1;
    }
    (i, n)
}

impl Map {
    fn max(&self) -> Pos {
        Pos {
            x: self.points.keys().map(|Pos { x, .. }| *x).max().unwrap(),
            y: self.points.keys().map(|Pos { y, .. }| *y).max().unwrap(),
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = self.max();
        for y in 1..=max.y {
            for x in 1..=max.x {
                let pos = Pos { x, y };
                match self.points.get(&pos) {
                    Some(cost) => write!(f, "{}", cost)?,
                    None => write!(f, " ")?,
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
impl Pos {
    fn adjacents(&self) -> impl Iterator<Item = Pos> {
        [
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
        ]
        .into_iter()
    }

    fn div(&self) -> (Self, Self) {
        let x = div(self.x);
        let y = div(self.y);

        (Self { x: x.0, y: y.0 }, Self { x: x.1, y: y.1 })
    }
}

impl std::str::FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = HashMap::new();

        for (y, l) in s.lines().enumerate() {
            for (x, ch) in l.trim().bytes().enumerate() {
                let ch = ch as u32 - '0' as u32;
                let pos = Pos {
                    x: (x + 1) as _,
                    y: (y + 1) as _,
                };
                // println!("{:?} = {} ({})", pos, ch, ch as char);
                points.insert(pos, ch as u32);
            }
        }

        Ok(Self { points })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG: &'static str = "\
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581\
    ";

    #[test]
    fn test_part1() {
        let lines = EG.parse().unwrap();
        assert_eq!(part1(&lines), 40);
    }

    #[test]
    fn test_wrap() {
        assert_eq!(wrap(1), 1);
        assert_eq!(wrap(8), 8);
        assert_eq!(wrap(9), 9);
        assert_eq!(wrap(10), 1);
        assert_eq!(wrap(12), 3);
        assert_eq!(wrap(15), 6);
        assert_eq!(wrap(18), 9);
        assert_eq!(wrap(19), 1);
        assert_eq!(wrap(20), 2);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(1), (1, 0));
        assert_eq!(div(9), (9, 0));
        assert_eq!(div(10), (10, 0));
        assert_eq!(div(11), (1, 1));
        assert_eq!(div(19), (9, 1));
        assert_eq!(div(20), (10, 1));
        assert_eq!(div(21), (1, 2));
    }

    #[test]
    fn test_part2() {
        let lines = EG.parse().unwrap();
        assert_eq!(part2(&lines), 315);
        assert!(false);
    }
}
