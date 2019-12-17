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
    min: Coord,
    max: Coord,
}

struct Mark {
    visited: HashSet<usize>,
}

fn minmax(map: HashMap<Coord, Tile>) -> (Coord, Coord) {
    map
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

impl View {
    fn parse(output: Vec<Word>) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut map = HashMap::new();

        for i in output {
            let ch = match char::from_u32(i as u32) {
                Some(x) => x,
                None => panic!("invalid digit {}", i),
            };

            if ch == '\n' {
                x = 0;
                y += 1;
            } else {
                map.insert(Coord { x, y }, Tile::from(ch));
                x += 1;
            }
        }

        let (min, max) = minmax(&map);

        Self {
            map,
            min,
            max,
        }
    }

    fn minmax(&self) -> (Coord, Coord) {
        minmax(&self.map)
    }

    fn is_scaffold(&self, coord: &Coord) -> bool {
        match self.map.get(coord) {
            Some(Tile::Scaffold) => true,
            Some(Tile::Robot(_)) => true,
            _ => false,
        }
    }

    fn count_overlaps(&self) -> isize {
        //let (min, max) = self.minmax();
        let (min, max) = (self.min, self.max);
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

    fn walk_direction(&self, from: &Coord, next: &Coord) {
        if !self.in_bounds(next) {
            return;
        }
        if ! self.is_scaffold(next) {
            return;
        }
    }

    fn walk(&self, from: &Coord) -> Vec<Route> {
        let Coord { x, y } = from;
        let above = Coord { x, y: y - 1 };
        let below = Coord { x, y: y + 1 };
        let left  = Coord { x: x - 1, y };
        let right = Coord { x: x + 1, y };

        let maybes = [
            self.try_direction(&above),
            self.try_direction(&below),
            self.try_direction(&left),
            self.try_direction(&right),
        ];

        for subroute in maybes {
            let subroute = match subroute {
                Some(sr) => sr,
                None => continue,
            };

        }
    }

    fn possible_routes(&self) -> Vec<Route> {
        let mut markedmap = HashMap::<Coord, Mark>::new();

        /*
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let c = Coord { x, y };
                match self.map.get(&c) {
                    Some(tile) => {
                        markedmap.insert(c, Mark {
                            tile,
                            visited: HashSet::new(),
                        });
                    }
                    None => {}
                }
            }
        }
        */
        let mut robot_coord = None;
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                match self.map.get(&Coord { x, y }) {
                    Some(Tile::Robot(_)) => {
                        robot_coord = Some(Coord { x, y });
                    },
                    _ => {},
                }
            }
        }

        self.walk(&robot_coord.expect("couldn't find robot"))
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

#[allow(dead_code)]
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

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = bytes("./input-day17")?;

    assert_eq!(bytes[0], 1);
    bytes[0] = 2;

    let mut machine = IntCodeMachine::new(&bytes, false);

    let output = machine.interpret_async(&mut vec![]);
    let view = View::parse(output);

    let routes = view.possible_routes();

    println!("{:?}", view);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //part1()?;
    part2()?;

    Ok(())
}
