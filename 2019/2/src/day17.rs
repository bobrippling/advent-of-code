use std::char;

mod parse;
use parse::bytes;

mod lib;
use lib::{IntCodeMachine, Word};
use std::collections::{HashMap/*, HashSet*/};

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

struct View {
    map: HashMap<Coord, Tile>,
    //markedmap: HashMap<Coord, Mark>,

    min: Coord,
    max: Coord,
    //nextid: usize,
}

/*struct Mark {
    visited: HashSet<usize>,
}*/

//struct Route(Vec<Coord>);

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

fn minmax(map: &HashMap<Coord, Tile>) -> (Coord, Coord) {
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

    /*
    fn walk_direction(
        &self,
        id: usize,
        from: &Coord,
        next: &Coord,
        prevroute: &Route
    ) -> Option<Route> {
        if !self.in_bounds(next) {
            return;
        }

        if !self.is_scaffold(next) {
            return;
        }

        match self.markedmap.get(next) {
            Some(Mark { ref mut visited }) => {
                if visited.get(&id) {
                    return;
                }

                visited.insert(id);
            },
            _ => {},
        }

        // direction ok, not been there before
        let forward_path = self.split(next);

        Route::new(&prevroute, next, forward_path)
    }

    fn split(&self, from: &Coord) -> Vec<Route> {
        let Coord { x, y } = from;

        let above = Coord { x, y: y - 1 };
        let below = Coord { x, y: y + 1 };
        let left  = Coord { x: x - 1, y };
        let right = Coord { x: x + 1, y };

        let maybes = [
            self.walk_direction(from, &above),
            self.walk_direction(from, &below),
            self.walk_direction(from, &left),
            self.walk_direction(from, &right),
        ];

        for subroute in maybes {
            let subroute = match subroute {
                Some(x) => x,
                None => continue,
            };


        }
    }

    fn possible_routes(&self) -> Vec<Route> {
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

        let mut robot_coord = None;
        'outer:
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                match self.map.get(&Coord { x, y }) {
                    Some(Tile::Robot(_)) => {
                        robot_coord = Some(Coord { x, y });
                        break 'outer;
                    },
                    _ => {},
                }
            }
        }

        self.split(&robot_coord.expect("couldn't find robot"))
    }
    */
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

    let mut input = Vec::new();

    input.extend_from_slice(&[
        'A' as Word,
        ',' as Word,
        'B' as Word,
        ',' as Word,
        'A' as Word,
        ',' as Word,
        'B' as Word,
        ',' as Word,
        'C' as Word,
        ',' as Word,
        'C' as Word,
        ',' as Word,
        'B' as Word,
        ',' as Word,
        'A' as Word,
        ',' as Word,
        'B' as Word,
        ',' as Word,
        'C' as Word,
        '\n' as Word,
    ]);
    input.extend_from_slice(&[
        'L' as Word, ',' as Word, '8' as Word, ',' as Word, 'R' as Word, ',' as Word, '1' as Word, '2' as Word, ',' as Word, 'R' as Word, ',' as Word, '1' as Word, '2' as Word, ',' as Word, 'R' as Word, ',' as Word, '1' as Word, '0' as Word, '\n' as Word,
        'R' as Word, ',' as Word, '1' as Word, '0' as Word, ',' as Word, 'R' as Word, ',' as Word, '1' as Word, '2' as Word, ',' as Word, 'R' as Word, ',' as Word, '1' as Word, '0' as Word, '\n' as Word,
        'L' as Word, ',' as Word, '1' as Word, '0' as Word, ',' as Word, 'R' as Word, ',' as Word, '1' as Word, '0' as Word, ',' as Word, 'L' as Word, ',' as Word, '6' as Word, '\n' as Word,
        'n' as Word, '\n' as Word,
    ]);

    let output = machine.interpret_async(&mut input);

    //let view = View::parse(output);

    //let routes = view.possible_routes();

    println!("{:?}", output);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //part1()?;
    part2()?;

    Ok(())
}
