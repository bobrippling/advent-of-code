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

use std::fs;
use std::collections::HashMap;

mod lib;
use lib::{IntCodeMachine, Word, State as ICMState};

struct Game {
    machine: IntCodeMachine,
    screen: HashMap<Coord, Tile>,
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Coord {
    x: Word,
    y: Word,
}

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizPaddle,
    Ball,
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

impl Game {
    fn new(bytes: &[Word]) -> Self {
        let machine = IntCodeMachine::new(bytes, false);

        Game {
            machine,
            screen: HashMap::<Coord, Tile>::new(),
        }
    }

    fn is_active(&self) -> bool {
        match self.machine.state() {
            ICMState::Running => true,
            ICMState::Halted => false,
        }
    }

    fn run(&mut self) {
        let outputs = self.machine.interpret_async(&mut vec![]);

        assert_eq!(outputs.len() % 3, 0);

        println!("outputs: {}", outputs.len());

        for i in (0..outputs.len()).step_by(3) {
            let slice = (
                outputs[i],
                outputs[i+1],
                outputs[i+2],
            );

            println!("output slice: {:?}", slice);

            let at = Coord {
                x: slice.0,
                y: slice.1,
            };
            let tile = Tile::from(slice.2);

            self.screen.insert(at, tile);
        }
    }

    fn run_til_end(&mut self) {
        while self.is_active() {
            self.run()
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input-day13")?;
    let bytes = s
        .trim_end()
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<Word>, _>>()?;

    let mut game = Game::new(&bytes);
    game.run_til_end();

    println!("{}",
        game.screen
            .values()
            .filter(|&v| v == &Tile::Block)
            .count());

    Ok(())
}
