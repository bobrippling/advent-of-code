// On the way to Jupiter, you're pulled over by the Space Police.
//
// "Attention, unmarked spacecraft! You are in violation of Space Law! All spacecraft must have a clearly visible registration identifier! You have 24 hours to comply or be sent to Space Jail!"
//
// Not wanting to be sent to Space Jail, you radio back to the Elves on Earth for help. Although it takes almost three hours for their reply signal to reach you, they send instructions for how to power up the emergency hull painting robot and even provide a small Intcode program (your puzzle input) that will cause it to paint your ship appropriately.
//
// There's just one problem: you don't have an emergency hull painting robot.
//
// You'll need to build a new emergency hull painting robot. The robot needs to be able to move around on the grid of square panels on the side of your ship, detect the color of its current panel, and paint its current panel black or white. (All of the panels are currently black.)
//
// The Intcode program will serve as the brain of the robot. The program uses input instructions to access the robot's camera: provide 0 if the robot is over a black panel or 1 if the robot is over a white panel. Then, the program will output two values:
//
//     First, it will output a value indicating the color to paint the panel the robot is over: 0 means to paint the panel black, and 1 means to paint the panel white.
//     Second, it will output a value indicating the direction the robot should turn: 0 means it should turn left 90 degrees, and 1 means it should turn right 90 degrees.
//
// After the robot turns, it should always move forward exactly one panel. The robot starts facing up.
//
// The robot will continue running for a while like this and halt when it is finished drawing. Do not restart the Intcode computer inside the robot during this process.
//
// For example, suppose the robot is about to start running. Drawing black panels as ., white panels as #, and the robot pointing the direction it is facing (< ^ > v), the initial state and region near the robot looks like this:
//
// .....
// .....
// ..^..
// .....
// .....
//
// The panel under the robot (not visible here because a ^ is shown instead) is also black, and so any input instructions at this point should be provided 0. Suppose the robot eventually outputs 1 (paint white) and then 0 (turn left). After taking these actions and moving forward one panel, the region now looks like this:
//
// .....
// .....
// .<#..
// .....
// .....
//
// Input instructions should still be provided 0. Next, the robot might output 0 (paint black) and then 0 (turn left):
//
// .....
// .....
// ..#..
// .v...
// .....
//
// After more outputs (1,0, 1,0):
//
// .....
// .....
// ..^..
// .##..
// .....
//
// The robot is now back where it started, but because it is now on a white panel, input instructions should be provided 1. After several more outputs (0,1, 1,0, 1,0), the area looks like this:
//
// .....
// ..<#.
// ...#.
// .##..
// .....
//
// Before you deploy the robot, you should probably have an estimate of the area it will cover: specifically, you need to know the number of panels it paints at least once, regardless of color. In the example above, the robot painted 6 panels at least once. (It painted its starting panel twice, but that panel is still only counted once; it also never painted the panel it ended on.)
//
// Build a new emergency hull painting robot and run the Intcode program on it. How many panels does it paint at least once?

use std::fs;

mod lib;
use lib::{IntCodeMachine, Word, State as ICMState};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Colour {
    Black,
    White,
}

impl Colour {
    fn from(x: Word) -> Self {
        match x {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!(),
        }
    }

    fn to_word(self) -> Word {
        match self {
            Self::Black => 0,
            Self::White => 1,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl std::ops::AddAssign<Dir> for Coord {
    fn add_assign(&mut self, rhs: Dir) {
        *self = match rhs {
            Dir::Up => Coord::new(self.x, self.y - 1),
            Dir::Down => Coord::new(self.x, self.y + 1),
            Dir::Left => Coord::new(self.x - 1, self.y),
            Dir::Right => Coord::new(self.x + 1, self.y),
        };
    }
}

enum Turn {
    Left = 0,
    Right = 1,
}

impl Turn {
    fn from(x: Word) -> Self {
        match x {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl std::ops::AddAssign<Turn> for Dir {
    fn add_assign(&mut self, rhs: Turn) {
        *self = match self {
            Dir::Up => match rhs {
                Turn::Left => Dir::Left,
                Turn::Right => Dir::Right,
            },
            Dir::Left => match rhs {
                Turn::Left => Dir::Down,
                Turn::Right => Dir::Up,
            },
            Dir::Down => match rhs {
                Turn::Left => Dir::Right,
                Turn::Right => Dir::Left,
            },
            Dir::Right => match rhs {
                Turn::Left => Dir::Up,
                Turn::Right => Dir::Down,
            },
        }
    }
}

#[derive(PartialEq)]
enum RobotState {
    Painting,
    Done,
}

type PaintGrid = HashMap<Coord, Colour>;

struct Robot {
    brain: IntCodeMachine,
    painted: PaintGrid,

    location: Coord,
    facing: Dir,

    uniq_painted: usize,
}

impl Robot {
    fn run(&mut self) -> RobotState {
        if self.brain.state() == ICMState::Halted {
            return RobotState::Done;
        }

        let colour = match self.painted.get(&self.location) {
            Some(&c) => c,
            None => Colour::Black,
        };

        //println!("current colour: {:?}", colour);

        let output = self.brain.interpret_async(&mut vec![colour.to_word()]);

        assert_eq!(output.len(), 2);
        let new_colour = Colour::from(output[0]);
        let turn = Turn::from(output[1]);

        self.paint(new_colour);
        self.turn_and_move(turn);

        RobotState::Painting
    }

    fn paint(&mut self, new_colour: Colour) {
        let old_colour = self.painted.insert(self.location, new_colour);

        if old_colour.is_none() {
            self.uniq_painted += 1;
        }
    }

    fn turn_and_move(&mut self, turn: Turn) {
        self.facing += turn;
        self.location += self.facing;
    }
}

fn show_paint(painted: &PaintGrid) {
    let min = Coord {
        x: painted.keys().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
        y: painted.keys().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
    };
    let max = Coord {
        x: painted.keys().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x,
        y: painted.keys().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y,
    };

    println!("painted, min: {:?}, max: {:?}", min, max);
    for y in min.y .. max.y {
        for x in min.x .. max.x {
            let colour = painted
                   .get(&Coord { x, y })
                   .map(|&c| c)
                   .unwrap_or(Colour::Black);

            let s = match colour {
                Colour::Black => ".",
                Colour::White => "\x1b[32m#\x1b[0m",
            };

            print!("{}", s);
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input-day11")?;
    let bytes = s // TODO: factor
        .trim_end()
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<Word>, _>>()?;

    let mut robot = Robot {
        brain: IntCodeMachine::new(&bytes, false),
        painted: Default::default(),

        location: Coord::new(0, 0),
        facing: Dir::Up,

        uniq_painted: 0,
    };

    while robot.run() == RobotState::Painting {}

    println!("{} {}",
             robot.painted.keys().count(),
             robot.uniq_painted);

    //show_paint(&robot.painted);

    Ok(())
}
