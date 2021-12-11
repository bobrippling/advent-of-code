use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let octos = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&octos));
    println!("Part 2: {}", part2(&octos));

    Ok(())
}

#[derive(Clone)]
struct Octos {
    grid: HashMap<Pos, Octo>,
    flashes: u64,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Clone)]
struct Octo(u8);

fn part1(octos: &Octos) -> u64 {
    let mut octos = octos.clone();

    for _ in 0..100 {
        octos.step();
    }

    octos.flashes
}

fn part2(octos: &Octos) -> u64 {
    let mut octos = octos.clone();

    for step in 1.. {
        octos.step();
        if octos.all_zero() {
            return step;
        }
    }

    unreachable!()
}

impl Octos {
    fn step(&mut self) {
        let max = self.max();

        for y in 0..=max.y {
            for x in 0..=max.x {
                let pos = Pos { x, y };

                let octo = self.grid.get_mut(&pos).unwrap();
                octo.inc();
            }
        }

        let mut flashers = HashSet::new();
        let mut recheck = true;

        while recheck {
            recheck = false;

            for y in 0..=max.y {
                for x in 0..=max.x {
                    let pos = Pos { x, y };

                    if flashers.contains(&pos) {
                        continue;
                    }

                    let octo = self.grid.get_mut(&pos).unwrap();

                    if octo.energy() > 9 {
                        self.flashes += 1;

                        flashers.insert(pos);

                        for adj in pos.adjacents() {
                            if let Some(octo) = self.grid.get_mut(&adj) {
                                octo.inc();
                                recheck = true;
                            }
                        }
                    }
                }
            }
        }

        for pos in flashers {
            let octo = self.grid.get_mut(&pos).unwrap();
            octo.reset();
        }
    }

    fn all_zero(&self) -> bool {
        self.grid.values().all(|octo| octo.energy() == 0)
    }

    fn max(&self) -> Pos {
        Pos {
            x: self.grid.keys().map(|Pos { x, .. }| *x).max().unwrap(),
            y: self.grid.keys().map(|Pos { y, .. }| *y).max().unwrap(),
        }
    }
}

impl Octo {
    fn inc(&mut self) {
        self.0 += 1;
    }

    fn energy(&self) -> u8 {
        self.0
    }

    fn reset(&mut self) {
        self.0 = 0;
    }
}

impl Pos {
    fn adjacents(self) -> impl Iterator<Item = Pos> {
        [
            Pos {
                x: self.x - 1,
                y: self.y - 1,
            },
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y - 1,
            },
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x - 1,
                y: self.y + 1,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
        .into_iter()
    }
}

impl std::fmt::Debug for Octos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = self.max();
        for y in 0..=max.y {
            for x in 0..=max.x {
                let pos = Pos { x, y };
                let octo = self.grid.get(&pos).unwrap();

                write!(f, "{}", octo.energy())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Octos {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = HashMap::new();

        s.lines().enumerate().map(|(y, l)| -> Result<(), &'static str>{
            l.trim().chars().enumerate().map(|(x, ch)| {
                let i = match ch {
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
                    _ => return Err("unknown char"),
                };

                let pos = Pos {
                    x: x as _,
                    y: y as _,
                };
                grid.insert(pos, Octo(i));
                Ok(())
            }).collect::<Result<Vec<_>, _>>()?;
            Ok(())
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(Self { grid, flashes: 0 })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG: &'static str = "\
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526\
    ";

    #[test]
    fn test_part1() {
        let octos = EG.parse().unwrap();
        assert_eq!(part1(&octos), 1656);
    }

    #[test]
    fn test_part2() {
        let octos = EG.parse().unwrap();
        assert_eq!(part2(&octos), 195);
    }
}
