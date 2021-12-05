use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &Lines) -> usize {
    let grid = lines.to_grid();

    grid.values()
        .filter(|&&v| v >= 2)
        .count()
}

fn part2(lines: &Lines) -> usize {
    let grid = lines.to_grid_diagonal();

    grid.values()
        .filter(|&&v| v >= 2)
        .count()
}

struct Lines(Vec<(Pos, Pos)>);

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
fn print_grid(grid: &HashMap<Pos, i32>) {
    let keys = grid.keys().collect::<Vec<_>>();

    let x2 = keys.iter().map(|Pos { x, .. }| x).copied().max().unwrap();
    let y2 = keys.iter().map(|Pos { y, .. }| y).copied().max().unwrap();

    for y in 0..=y2 {
        for x in 0..=x2 {
            let pos = Pos {x ,y};
            let v = grid.get(&pos).copied().unwrap_or(0);

            let s = if v == 0 { '.'.into() } else { v.to_string() };

            print!("{}", s)
        }
        println!();
    }

}

impl Lines {
    fn to_grid(&self) -> HashMap<Pos, i32> {
        let mut grid = HashMap::new();

        for line in &self.0 {
            let (a, b) = line;
            if a.x != b.x && a.y != b.y {
                continue;
            }

            let min = a.min(b);
            let max = a.max(b);

            for x in min.x..=max.x {
                for y in min.y..=max.y {
                    let pos = Pos { x, y };
                    *grid.entry(pos).or_insert(0) += 1;
                }
            }
        }

        grid
    }

    fn to_grid_diagonal(&self) -> HashMap<Pos, i32> {
        let mut grid = HashMap::new();

        for line in &self.0 {
            let (a, b) = line;
            // println!("doing {:?} to {:?}", a, b);

            if a.x == b.x || a.y == b.y {
                let min = a.min(b);
                let max = a.max(b);

                for x in min.x..=max.x {
                    for y in min.y..=max.y {
                        let pos = Pos { x, y };
                        *grid.entry(pos).or_insert(0) += 1;
                    }
                }
            } else {
                // diagonal
                let start = *a;
                let end = *b;
                let dir_x = if start.x < end.x { 1 } else { -1 };
                let dir_y = if start.y < end.y { 1 } else { -1 };
                let mut pos = start;

                loop {
                    *grid.entry(pos).or_insert(0) += 1;
                    if pos == end {
                        break;
                    }
                    pos.x += dir_x;
                    pos.y += dir_y;
                }
            }
        }

        grid
    }
}

impl Pos {
    fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
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
}
