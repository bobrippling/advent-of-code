use std::collections::{HashMap, HashSet};

struct Cave {
    points: HashMap<Pos, u8>,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cave = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&cave));
    println!("Part 2: {}", part2(&cave));

    Ok(())
}

fn part1(cave: &Cave) -> i32 {
    let low_points = cave.low_points();

    low_points.into_iter().map(|(_, val)| (val + 1) as i32).sum()
}

fn part2(cave: &Cave) -> u32 {
    let mut basins = cave.basins().collect::<Vec<_>>();

    basins.sort();

    basins.into_iter().rev().take(3).product()
}

impl Cave {
    fn low_points(&self) -> Vec<(Pos, u8)> {
        let max = self.max();
        let mut low_points = Vec::new();

        for y in 0..=max.y {
            for x in 0..=max.x {
                let pt = Pos { x, y };
                let val = self.points.get(&pt).copied().unwrap();

                if self.adjacent_pts(pt).all(|adj_val| adj_val > val) {
                    low_points.push((pt, val));
                }
            }
        }

        low_points
    }

    fn basins(&self) -> impl Iterator<Item = u32> + '_ {
        let low_points = self.low_points();

        low_points.into_iter().map(|(pos, _)| self.basin(pos))
    }

    fn basin(&self, pos: Pos) -> u32 {
        let mut seen = HashSet::new();

        self.basin_r(pos, &mut seen)
    }

    fn basin_r(&self, pos: Pos, seen: &mut HashSet<Pos>) -> u32 {
        if seen.contains(&pos) {
            return 0;
        }
        seen.insert(pos);

        let mut total = match self.points.get(&pos) {
            Some(&9) => return 0,
            Some(_) => 1 as u32,
            None => return 0,
        };

        for point in pos.adjacents() {
            total += self.basin_r(point, seen);
        }

        total
    }

    fn max(&self) -> Pos {
        Pos {
            x: self.points.keys().map(|Pos { x, .. }| *x).max().unwrap(),
            y: self.points.keys().map(|Pos { y, .. }| *y).max().unwrap(),
        }
    }

    fn adjacent_pts(&self, pos: Pos) -> impl Iterator<Item = u8> + '_ {
        pos.adjacents().filter_map(|p| self.points.get(&p)).copied()
    }
}

impl Pos {
    fn adjacents(self) -> impl Iterator<Item = Pos> {
        [
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
        ]
        .into_iter()
    }
}

impl std::str::FromStr for Cave {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = HashMap::new();

        for (y, line) in s.lines().map(str::trim).enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = Pos {
                    x: x as _,
                    y: y as _,
                };

                let n = match ch {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => panic!("char '{}'", ch),
                };

                points.insert(pos, n);
            }
        }

        Ok(Self { points })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG: &'static str = "\
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn test_part1() {
        let lines = EG.parse().unwrap();
        assert_eq!(part1(&lines), 15);
    }

    #[test]
    fn test_part2() {
        let lines = EG.parse().unwrap();
        assert_eq!(part2(&lines), 1134);
    }
}
