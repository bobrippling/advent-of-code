use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[derive(Clone)]
struct Input {
    enhancement: [bool; 512],
    image: HashSet<Pos>,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

fn part1(input: &Input) -> i64 {
    let mut input = input.clone();

    for step in 0..2 {
        input.enhance(step % 2 == 1);
    }

    #[cfg(test)]
    {
        println!("{}", input);
    }

    input.image.len() as _
}

fn part2(input: &Input) -> i64 {
    let mut input = input.clone();

    for step in 0..50 {
        input.enhance(step % 2 == 1);
    }

    input.image.len() as _
}

impl Input {
    fn enhance(&mut self, mut default: bool) {
        if !self.enhancement[0] {
            // default is false always for this case, otherwise, it toggles the infinite grid
            default = false;
        }

        let mut out = HashSet::new();
        let (min, max) = self.minmax();

        for y in min.y - 1..=max.y + 1 {
            for x in min.x - 1..=max.x + 1 {
                let pos = Pos { x, y };
                let light = self.enhancement_for(&pos, (&min, &max), default);
                if light {
                    out.insert(pos);
                }
            }
        }

        self.image = out;
    }

    fn minmax(&self) -> (Pos, Pos) {
        (
            Pos {
                x: self.image.iter().map(|p| p.x).min().unwrap(),
                y: self.image.iter().map(|p| p.y).min().unwrap(),
            },
            Pos {
                x: self.image.iter().map(|p| p.x).max().unwrap(),
                y: self.image.iter().map(|p| p.y).max().unwrap(),
            },
        )
    }

    fn enhancement_for(&self, pos: &Pos, minmax: (&Pos, &Pos), default: bool) -> bool {
        let i = self.index_for(pos, minmax, default);

        self.enhancement[i as usize]
    }

    fn index_for(&self, pos: &Pos, (min, max): (&Pos, &Pos), default: bool) -> u16 {
        pos.adjacents()
            .map(|p| {
                if default {
                    self.image.contains(&p)
                        || p.x > max.x
                        || p.x < min.x
                        || p.y > max.y
                        || p.y < min.y
                } else {
                    self.image.contains(&p)
                }
            })
            .map(|b| b as u16)
            .reduce(|acc, b| (acc << 1) | b)
            .unwrap()
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self.minmax();

        for y in min.y - 1..=max.y + 1 {
            for x in min.x - 1..=max.x + 1 {
                let p = Pos { x, y };
                write!(f, "{}", if self.image.contains(&p) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Pos {
    fn adjacents(&self) -> impl Iterator<Item = Pos> {
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

impl std::str::FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let enhancement: Vec<_> = s
            .lines()
            .take(1)
            .flat_map(|l| {
                l.chars().map(|ch| {
                    Ok(match ch {
                        '#' => true,
                        '.' => false,
                        _ => return Err("unknown enhancement char"),
                    })
                })
            })
            .collect::<Result<_, _>>()?;

        let mut image = HashSet::new();
        for (y, l) in s.lines().skip(2).map(str::trim).enumerate() {
            for (x, ch) in l.chars().enumerate() {
                match ch {
                    '#' => {
                        let pos = Pos {
                            x: x as _,
                            y: y as _,
                        };
                        image.insert(pos);
                    }
                    '.' => {}
                    _ => return Err("unknown image char"),
                }
            }
        }

        Ok(Self {
            enhancement: enhancement
                .try_into()
                .map_err(|_| "invalid enhancement length")?,
            image,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = EG.parse().unwrap();

        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = EG.parse().unwrap();

        assert_eq!(part2(&input), 3351);
    }

    static EG: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

    #..#.
    #....
    ##..#
    ..#..
    ..###\
    ";
}
