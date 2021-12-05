use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &Lines) -> usize {
    let grid = lines.to_grid(Grid::HorzVertOnly);

    grid.values().filter(|&&v| v >= 2).count()
}

fn part2(lines: &Lines) -> usize {
    let grid = lines.to_grid(Grid::IncludeDiagonal);

    grid.values().filter(|&&v| v >= 2).count()
}

struct Lines(Vec<(Pos, Pos)>);

enum Grid {
    HorzVertOnly,
    IncludeDiagonal,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Lines {
    fn to_grid(&self, mode: Grid) -> HashMap<Pos, i32> {
        let mut grid = HashMap::new();

        for line in &self.0 {
            let &(ref a, b) = line;

            match mode {
                Grid::HorzVertOnly => {
                    if a.x != b.x && a.y != b.y {
                        continue;
                    }
                }
                Grid::IncludeDiagonal => {}
            }

            for pos in a.to(b) {
                *grid.entry(pos).or_insert(0) += 1;
            }
        }

        grid
    }
}

impl Pos {
    fn to(mut self, target: Self) -> impl Iterator<Item = Pos> {
        use std::{iter, ops};

        let dir = Dir::from(self, target);
        let mut done = false;

        return iter::from_fn(move || {
            if done {
                return None;
            }
            if self == target {
                done = true;
            }

            let ret = self;
            self += dir;
            Some(ret)
        });

        #[derive(Clone, Copy)]
        struct Dir {
            x: i32,
            y: i32,
        }

        impl Dir {
            fn from(from: Pos, to: Pos) -> Self {
                fn increment_for(from: i32, to: i32) -> i32 {
                    use std::cmp::Ordering::*;

                    match from.cmp(&to) {
                        Less => 1,
                        Equal => 0,
                        Greater => -1,
                    }
                }

                Self {
                    x: increment_for(from.x, to.x),
                    y: increment_for(from.y, to.y),
                }
            }
        }

        impl ops::AddAssign<Dir> for Pos {
            fn add_assign(&mut self, dir: Dir) {
                self.x += dir.x;
                self.y += dir.y;
            }
        }
    }
}

impl std::str::FromStr for Lines {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Lines(
            s.lines()
                .map(|l| {
                    if let [from, to] = l.split(" -> ").collect::<Vec<_>>()[..] {
                        Ok((from.parse()?, to.parse()?))
                    } else {
                        Err("incorrect line format")
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl std::str::FromStr for Pos {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [x, y] = s.split(',').collect::<Vec<_>>()[..] {
            let invalid_num = |_| "invalid number";

            let x = x.trim().parse().map_err(invalid_num)?;
            let y = y.parse().map_err(invalid_num)?;

            Ok(Self { x, y })
        } else {
            Err("incorrect position format")
        }
    }
}

#[allow(dead_code)]
fn print_grid(grid: &HashMap<Pos, i32>) {
    let keys = grid.keys().collect::<Vec<_>>();

    let x2 = keys.iter().map(|Pos { x, .. }| x).copied().max().unwrap();
    let y2 = keys.iter().map(|Pos { y, .. }| y).copied().max().unwrap();

    for y in 0..=y2 {
        for x in 0..=x2 {
            let pos = Pos { x, y };
            let v = grid.get(&pos).copied().unwrap_or(0);

            let s = if v == 0 { '.'.into() } else { v.to_string() };

            print!("{}", s)
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG: &'static str = "\
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2\
    ";

    #[test]
    fn test_part1() {
        let lines = EG.parse().unwrap();

        assert_eq!(part1(&lines), 5);
    }

    #[test]
    fn test_part2() {
        let lines = EG.parse().unwrap();

        assert_eq!(part2(&lines), 12);
    }

    #[test]
    fn test_pos() {
        let a = Pos { x: 1, y: 2 };
        let b = Pos { x: 1, y: 5 };

        let iter = a.to(b);
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![
                Pos { x: 1, y: 2 },
                Pos { x: 1, y: 3 },
                Pos { x: 1, y: 4 },
                Pos { x: 1, y: 5 },
            ]
        );
    }
}
