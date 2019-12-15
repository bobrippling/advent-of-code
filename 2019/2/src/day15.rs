#![allow(unused_imports)]
use std::fs;

use std::io::{self, Write, Read};
use std::process::Command;

mod lib;
use lib::{IntCodeMachine, Word, State as ICMState};
use std::collections::HashMap;

use std::convert::TryInto;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

//impl std::cmp::Ord for Coord {
//}

impl std::ops::AddAssign<Dir> for Coord {
    fn add_assign(&mut self, rhs: Dir) {
        *self = match rhs {
            Dir::North => Coord::new(self.x, self.y - 1),
            Dir::South => Coord::new(self.x, self.y + 1),
            Dir::West => Coord::new(self.x - 1, self.y),
            Dir::East => Coord::new(self.x + 1, self.y),
        };
    }
}

impl std::ops::Add<Dir> for Coord {
    type Output = Self;
    fn add(mut self, rhs: Dir) -> Self {
        self += rhs;
        self
    }
}

/*
#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl std::ops::AddAssign<Turn> for Dir {
    fn add_assign(&mut self, rhs: Turn) {
        *self = match self {
            Dir::North => match rhs {
                Turn::Left => Dir::West,
                Turn::Right => Dir::East,
            },
            Dir::West => match rhs {
                Turn::Left => Dir::South,
                Turn::Right => Dir::North,
            },
            Dir::South => match rhs {
                Turn::Left => Dir::East,
                Turn::Right => Dir::West,
            },
            Dir::East => match rhs {
                Turn::Left => Dir::North,
                Turn::Right => Dir::South,
            },
        }
    }
}
*/

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl From<Dir> for i64 {
    fn from(dir: Dir) -> i64 {
        match dir {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum GridEnt {
    Wall,
    Ok,
    Found,
    Unknown,
}

impl From<i64> for GridEnt {
    fn from(e: i64) -> GridEnt {
        match e {
            0 => GridEnt::Wall,
            1 => GridEnt::Ok,
            2 => GridEnt::Found,
            _ => panic!("can't convert {} to grident", e),
        }
    }
}

type Grid = HashMap<Coord, GridEnt>;

struct Robot {
    machine: IntCodeMachine,

    grid: Grid,

    coord: Coord,
}

impl Robot {
    fn draw(&self) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        for coord in self.grid.keys() {
            min_x = min_x.min(coord.x);
            max_x = max_x.max(coord.x);
            min_y = min_y.min(coord.y);
            max_y = max_y.max(coord.y);
        }

        println!(
            "------- min = {:?}, max= {:?}",
            Coord { x: min_x, y: min_y },
            Coord { x: max_x, y: max_y });

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let ent = *self.grid.get(&Coord {x,y}).unwrap_or(&GridEnt::Unknown);

                let ch = if (Coord { x, y }) == self.coord {
                    "\x1b[33mo\x1b[0m"
                } else {
                    match ent {
                        GridEnt::Wall => "\x1b[32m#\x1b[0m",
                        GridEnt::Ok => " ",
                        GridEnt::Found => "\x1b[40m@\x10[0m",
                        GridEnt::Unknown => "\x1b[31m?\x1b[0m",
                    }
                };

                print!("{}",  ch);
            }
            print!("\n");
        }
    }

    fn run(&mut self) {
        let mut stty = Command::new("stty");
        stty.arg("-echo").arg("-icanon");
        stty.status().expect("stty failed");

        let stdout = io::stdout();
        let mut reader = io::stdin();
        let mut buffer = [0; 1]; // read exactly one byte

        loop {
            self.draw();
            let dir = loop {
                stdout.lock().flush().unwrap();
                reader.read_exact(&mut buffer).unwrap();

                match buffer[0] as char {
                    'h' => break Dir::West,
                    'l' => break Dir::East,
                    'k' => break Dir::North,
                    'j' => break Dir::South,
                    _ => {
                        eprintln!("invalid input");
                    },
                };
            };

            let output = self.machine.interpret_async(&mut vec![dir.into()]);
            assert_eq!(output.len(), 1);
            let answer: GridEnt = output[0].into();

            self.grid.insert(
                self.coord + dir,
                answer);

            match answer {
                GridEnt::Wall => {
                },
                GridEnt::Ok => {
                    self.coord += dir;
                },
                GridEnt::Found => {
                    self.coord += dir;
                    break;
                },
                GridEnt::Unknown => panic!(),
            }
        };
        println!("found at {:?}", self.coord);
    }

    fn auto(&mut self) {
        let mut dir = Dir::North;
        let mut moveback = false;

        loop {
            let output = self.machine.interpret_async(&mut vec![dir.into()]);
            assert_eq!(output.len(), 1);
            let answer: GridEnt = output[0].into();

            self.grid.insert(
                self.coord + dir,
                answer);

            match answer {
                GridEnt::Wall => {
                    if moveback {
                        moveback = false;
                        dir = -dir;
                    } else {

                    }
                },
                GridEnt::Ok => {
                    self.coord += dir;
                },
                GridEnt::Found => {
                    self.coord += dir;
                    break;
                },
                GridEnt::Unknown => panic!(),
            }
        };

        println!("found at {:?}", self.coord);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input-day15")?;
    let bytes = s // TODO: factor
        .trim_end()
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<Word>, _>>()?;

    let mut robot = Robot {
        machine: IntCodeMachine::new(&bytes, false),
        grid: Default::default(),
        coord: Coord { x: 0, y: 0 },
    };

    /*
    let lines = fs::read_to_string("./map-day15")?
        .split('\n')
        .map(|x| x.into())
        .collect::<Vec<String>>();

    for (y, line) in lines.iter().enumerate() {
        for x in 0..line.len() {
            let at = Coord{
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            let ch = line.as_bytes()[x];
            let ent = match ch {
                b'o' => {
                    // nothing
                    None
                },
                b'#' => Some(GridEnt::Wall),
                b' ' => Some(GridEnt::Ok),
                b'@' => Some(GridEnt::Found),
                b'?' => None, //Some(GridEnt::Unknown),
                _ => {
                    panic!("unknown grid ent {}", ch);
                }
            };

            match ent {
                Some(ge) => { robot.grid.insert(at, ge); }
                None => {},
            };
        }
    }
    */

    robot.run();

    Ok(())
}
