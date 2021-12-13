use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2:");
    part2(&lines);

    Ok(())
}

struct Input {
    paper: Paper,
    folds: Vec<Fold>,
}

#[derive(Clone)]
struct Paper {
    dots: HashSet<Pos>,
}

enum Fold {
    X(u32),
    Y(u32),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: u32,
    y: u32,
}

fn part1(input: &Input) -> u64 {
    let mut paper = input.paper.clone();

    paper.fold(&input.folds[0]);

    paper.dots.len() as _
}

fn part2(input: &Input) {
    let mut paper = input.paper.clone();

    for fold in &input.folds {
        paper.fold(fold);
    }

    println!("{}", paper);
}

impl Paper {
    fn fold(&mut self, fold: &Fold) {
        let mut taken = HashSet::new();

        match fold {
            &Fold::X(vert) => {
                for pos in &self.dots {
                    if pos.x >= vert {
                        taken.insert(*pos);
                    }
                }

                for pos in &taken {
                    self.dots.remove(pos);
                    self.dots.insert(pos.mirror_x(vert));
                }
            }
            &Fold::Y(horiz) => {
                for pos in &self.dots {
                    if pos.y >= horiz {
                        taken.insert(*pos);
                    }
                }

                for pos in &taken {
                    self.dots.remove(pos);
                    self.dots.insert(pos.mirror_y(horiz));
                }
            }
        }
    }

    fn max(&self) -> Pos {
        Pos {
            x: self.dots.iter().map(|&Pos { x, .. }| x).max().unwrap(),
            y: self.dots.iter().map(|&Pos { y, .. }| y).max().unwrap(),
        }
    }
}

impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = self.max();

        for y in 0..=max.y {
            for x in 0..=max.x {
                let pos = Pos { x, y };

                write!(f, "{}", if self.dots.contains(&pos) { '#' } else { ' ' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Pos {
    fn mirror_y(&self, y: u32) -> Self {
        assert!(self.y >= y);

        Self {
            x: self.x,
            y: y - (self.y - y),
        }
    }

    fn mirror_x(&self, x: u32) -> Self {
        assert!(self.x >= x);

        Self {
            x: x - (self.x - x),
            y: self.y,
        }
    }
}

impl std::str::FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("\n\n").collect();

        if let [dots, folds] = parts[..] {
            Ok(Self {
                paper: dots.parse()?,
                folds: folds
                    .lines()
                    .map(str::trim)
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()?,
            })
        } else {
            Err("wrong paragraph count")
        }
    }
}

impl std::str::FromStr for Paper {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            dots: s.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

impl std::str::FromStr for Fold {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("fold along ") {
            return Err("invalid prefix");
        }

        let n = s[13..].parse().map_err(|_| "invalid x/y number")?;

        Ok(match s.chars().nth(11) {
            Some('x') => Self::X(n),
            Some('y') => Self::Y(n),
            _ => return Err("invalid x/y type"),
        })
    }
}

impl std::str::FromStr for Pos {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split(',').collect();

        if let [x, y] = parts[..] {
            Ok(Self {
                x: x.parse().map_err(|_| "parse x")?,
                y: y.parse().map_err(|_| "parse y")?,
            })
        } else {
            Err("wrong split count for pos")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG: &'static str = "\
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5\
    ";

    #[test]
    fn test_part1() {
        let lines = EG.parse().unwrap();
        assert_eq!(part1(&lines), 17);
    }
}
