mod parse;
use parse::bytes;

mod lib;
use lib::{IntCodeMachine, Word};
use std::collections::{HashMap/*, HashSet*/};

mod d2;
use d2::{Coord};

fn scan_grid(bytes: &[Word], xlim: Word, ylim: Word) -> HashMap<Coord, Word> {
    let mut grid = HashMap::new();

    for y in 0..ylim {
        for x in 0..xlim {
            let mut machine = IntCodeMachine::new(bytes, false);

            let mut input = vec![x, y];
            let output = machine.interpret_async(&mut input);

            assert_eq!(output.len(), 1);
            grid.insert(Coord::new(x as isize, y as isize), output[0]);
        }
    }

    grid
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = bytes("./input-day19")?;

    let grid = scan_grid(&bytes, 50, 50);

    let affected = grid.values()
        .filter(|&&v| v == 1)
        .count();

    println!("{:?}", affected);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1()?;
    //part2()?;

    Ok(())
}
