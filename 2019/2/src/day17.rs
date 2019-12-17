use std::char;

mod parse;
use parse::bytes;

mod lib;
use lib::{IntCodeMachine, Word};
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Scaffold,
    Robot(Dir),
}

impl Tile {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Tile::Scaffold,
            '.' => Tile::Empty,
            '^' => Tile::Robot(Dir::North),
            '>' => Tile::Robot(Dir::East),
            '<' => Tile::Robot(Dir::West),
            'v' => Tile::Robot(Dir::South),
            _ => panic!("invalid ch {}", ch),
        }
    }

    fn to_ch(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Scaffold => '#',
            Tile::Robot(d) => match d {
                Dir::North => '^',
                Dir::East => '>',
                Dir::West => '<',
                Dir::South => 'v',
            }
        }
    }
}

struct View {
    map: HashMap<Coord, Tile>,
}

impl View {
    fn parse(output: Vec<Word>) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut map = HashMap::new();

        for i in output {
            let ch = char::from_u32(i as u32).expect("invalid digit");

            if ch == '\n' {
                x = 0;
                y += 1;
            } else {
                map.insert(Coord { x, y }, Tile::from(ch));
                x += 1;
            }
        }

        Self { map }
    }

    fn minmax(&self) -> (Coord, Coord) {
        self.map
            .keys()
            .fold(
                (Coord::zero(), Coord::zero()),
                |(min, max), Coord { x, y }| (
                    Coord {
                        x: *x.min(&min.x),
                        y: *y.min(&min.y),
                    },
                    Coord {
                        x: *x.max(&max.x),
                        y: *y.max(&max.y),
                    },
                ))
    }

    fn is_scaffold(&self, coord: &Coord) -> bool {
        match self.map.get(coord) {
            Some(Tile::Scaffold) => true,
            _ => false,
        }
    }

    fn count_overlaps(&self) -> isize {
        let (min, max) = self.minmax();
        let mut overlaps = 0;

        for y in min.y+1..max.y {
            for x in min.x+1..max.x {
                if !self.is_scaffold(&Coord { x, y }) {
                    continue;
                }

                let above = Coord { x, y: y - 1 };
                let below = Coord { x, y: y + 1 };
                let left  = Coord { x: x - 1, y };
                let right = Coord { x: x + 1, y };

                if self.is_scaffold(&above)
                    && self.is_scaffold(&below)
                    && self.is_scaffold(&left)
                    && self.is_scaffold(&right)
                {
                    overlaps += x * y;
                }
            }
        }

        overlaps
    }
}

impl std::fmt::Debug for View {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let (min, max) = self.minmax();

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let ch = self.map.get(&Coord { x, y })
                    .unwrap_or(&Tile::Empty)
                    .to_ch();

                write!(fmt, "{}", ch)?;
            }
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = bytes("./input-day17")?;

    let mut machine = IntCodeMachine::new(&bytes, false);

    let output = machine.interpret_async(&mut vec![]);
    //println!("output: {:?}", output);
    let view = View::parse(output);

    //println!("view:\n{:?}", view);

    println!("{}", view.count_overlaps());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1()?;

    Ok(())
}
