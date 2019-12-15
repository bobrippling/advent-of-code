// --- Day 13: Care Package ---
//
// As you ponder the solitude of space and the ever-increasing three-hour roundtrip for messages between you and Earth, you notice that the Space Mail Indicator Light is blinking. To help keep you sane, the Elves have sent you a care package.
//
// It's a new game for the ship's arcade cabinet! Unfortunately, the arcade is all the way on the other end of the ship. Surely, it won't be hard to build your own - the care package even comes with schematics.
//
// The arcade cabinet runs Intcode software like the game the Elves sent (your puzzle input). It has a primitive screen capable of drawing square tiles on a grid. The software draws tiles to the screen with output instructions: every three output instructions specify the x position (distance from the left), y position (distance from the top), and tile id. The tile id is interpreted as follows:
//
//     0 is an empty tile. No game object appears in this tile.
//     1 is a wall tile. Walls are indestructible barriers.
//     2 is a block tile. Blocks can be broken by the ball.
//     3 is a horizontal paddle tile. The paddle is indestructible.
//     4 is a ball tile. The ball moves diagonally and bounces off objects.
//
// For example, a sequence of output values like 1,2,3,6,5,4 would draw a horizontal paddle tile (1 tile from the left and 2 tiles from the top) and a ball tile (6 tiles from the left and 5 tiles from the top).
//
// Start the game. How many block tiles are on the screen when the game exits?
//
// --- Part Two ---
//
// The game didn't run because you didn't put in any quarters. Unfortunately, you did not bring any quarters. Memory address 0 represents the number of quarters that have been inserted; set it to 2 to play for free.
//
// The arcade cabinet has a joystick that can move left and right. The software reads the position of the joystick with input instructions:
//
//     If the joystick is in the neutral position, provide 0.
//     If the joystick is tilted to the left, provide -1.
//     If the joystick is tilted to the right, provide 1.
//
// The arcade cabinet also has a segment display capable of showing a single number that represents the player's current score. When three output instructions specify X=-1, Y=0, the third output instruction is not a tile; the value instead specifies the new score to show in the segment display. For example, a sequence of output values like -1,0,12345 would show 12345 as the player's current score.
//
// Beat the game by breaking all the blocks. What is your score after the last block is broken?

mod parse;
use parse::bytes;

use std::io::{self, Write, Read};
use std::collections::HashMap;
use std::process::Command;

mod lib;
use lib::{IntCodeMachine, Word, State as ICMState};

const ESC_UP: &str = "\x1b[A";

type Screen = HashMap<Coord, Tile>;

struct Game {
    machine: IntCodeMachine,
    save: (Vec<Word>, Screen),
    screen: Screen,
    score: Word,

    min_x: Word,
    max_x: Word,
    min_y: Word,
    max_y: Word,

    printed: bool,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Coord {
    x: Word,
    y: Word,
}

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizPaddle,
    Ball,
}

#[derive(Clone, Copy)]
enum Joystick {
    Neutral,
    Left,
    Right,
}

impl Joystick {
    fn to(self) -> Word {
        match self {
            Joystick::Neutral => 0,
            Joystick::Left => -1,
            Joystick::Right => 1,
        }
    }
}

impl std::fmt::Display for Joystick {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let c = match self {
            Joystick::Neutral => 'x',
            Joystick::Left => 'h',
            Joystick::Right => 'l',
        };
        write!(fmt, "{}", c)?;
        Ok(())
    }
}

impl Tile {
    fn from(i: Word) -> Self {
        match i {
            0 => Tile::Empty, // empty tile. No game object appears in this tile.
            1 => Tile::Wall, // wall tile. Walls are indestructible barriers.
            2 => Tile::Block, // block tile. Blocks can be broken by the ball.
            3 => Tile::HorizPaddle, // horizontal paddle tile. The paddle is indestructible.
            4 => Tile::Ball, // ball tile. The ball moves diagonally and bounces off objects.
            _ => {
                eprintln!("invalid tile {}", i);
                Tile::Empty
            }
        }
    }
}

fn save_input(j: Joystick) {
    eprintln!("{}", j);
}

impl Game {
    fn new(bytes: &[Word]) -> Self {
        let machine = IntCodeMachine::new(bytes, false);

        Game {
            machine,
            screen: HashMap::new(),
            save: (Vec::new(), HashMap::new()),

            score: 0,

            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,

            printed: false,
        }
    }

    fn is_active(&self) -> bool {
        match self.machine.state() {
            ICMState::Running => true,
            ICMState::Halted => false,
        }
    }

    fn run(&mut self, inputs: &mut Vec<Word>) {
        let outputs = self.machine.interpret_async(inputs);

        assert_eq!(outputs.len() % 3, 0);

        //println!("outputs: {}", outputs.len());

        for i in (0..outputs.len()).step_by(3) {
            let slice = (
                outputs[i],
                outputs[i+1],
                outputs[i+2],
            );

            //println!("output slice: {:?}", slice);

            let at = Coord {
                x: slice.0,
                y: slice.1,
            };

            if at.x == -1 && at.y == 0 {
                self.score = slice.2;
                continue;
            }

            let tile = Tile::from(slice.2);

            self.min_x = self.min_x.min(at.x);
            self.max_x = self.max_x.max(at.x);
            self.min_y = self.min_y.min(at.y);
            self.max_y = self.max_y.max(at.y);
            self.screen.insert(at, tile);
        }
    }

    fn run_til_end(&mut self) {
        while self.is_active() {
            self.run(&mut vec![])
        }
    }

    fn show(&mut self) {
        if self.printed {
            for _ in self.min_y..=self.max_y+1 {
                print!("{}", ESC_UP);
            }
        }

        println!("score {} {}\x1b[0K", self.score, if self.save.0.len() > 0 { "save present" } else { "" });
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let tile = self.screen.get(&Coord { x, y }).unwrap_or(&Tile::Empty);

                let c = match tile {
                    Tile::Empty => ' ',
                    Tile::Wall => '|',
                    Tile::Block => '#',
                    Tile::HorizPaddle => '_',
                    Tile::Ball => 'o',
                };

                print!("{}", c);
            }
            println!();
        }
        self.printed = true;
    }

    fn save(&mut self) {
        self.save.0.clear();
        self.save.0.extend_from_slice(self.machine.memory());
        self.save.1 = self.screen.clone();
    }

    fn load(&mut self) {
        self.machine.load_memory(&self.save.0);
        self.screen = self.save.1.clone();
    }

    fn interact(&mut self) {
        let mut stty = Command::new("stty");
        stty.arg("-echo").arg("-icanon");
        stty.status().expect("stty failed");

        let stdout = io::stdout();
        let mut reader = io::stdin();
        let mut buffer = [0; 1]; // read exactly one byte

        let mut inputs = Vec::new();
        loop {
            while self.is_active() {
                self.run(&mut inputs);

                let j = loop {
                    self.show();
                    stdout.lock().flush().unwrap();
                    reader.read_exact(&mut buffer).unwrap();

                    match buffer[0] as char {
                        'h' => break Joystick::Left,
                        'l' => break Joystick::Right,
                        'S' => {
                            self.save();
                        },
                        'L' => {
                            self.load();
                        },
                        _ => break Joystick::Neutral,
                    };
                };
                inputs.push(j.to());
                save_input(j);
            }

            if self.save.0.len() > 0 {
                self.load();
            } else {
                break;
            }
        }
    }
}

#[allow(dead_code)]
fn part1(bytes: &mut [Word]) {
    let mut game = Game::new(bytes);

    game.run_til_end();

    println!("{}",
        game.screen
            .values()
            .filter(|&v| v == &Tile::Block)
            .count());
}

#[allow(dead_code)]
fn part2(bytes: &mut [Word]) {
    bytes[0] = 2; // play for free
    let mut game = Game::new(bytes);
    game.interact();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = bytes("./input-day13")?;

    part1(&mut bytes.clone());
    part2(&mut bytes.clone());

    Ok(())
}
