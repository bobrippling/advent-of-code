mod parse;
use parse::bytes;

mod lib;
use lib::{IntCodeMachine, Word};

mod d2;
use d2::{Coord};

mod grid;
use grid::Grid;

fn get_coord(bytes: &[Word], &Coord { x, y }: &Coord) -> Word {
    let mut machine = IntCodeMachine::new(bytes, false);

    // FIXME: what's the diff between Word and typeof(Coord::x) ?
    let mut input = vec![x as Word, y as Word];
    let output = machine.interpret_async(&mut input);

    assert_eq!(output.len(), 1);
    output[0]
}

fn scan_grid(bytes: &[Word], xlim: Word, ylim: Word) -> Grid<Word> {
    let mut grid = Grid::new();

    for y in 0..ylim {
        for x in 0..xlim {
            let v = get_coord(bytes, &Coord { x: x as _, y: y as _ });
            grid.map.insert(Coord::new(x as isize, y as isize), v);
        }
    }

    grid
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = bytes("./input-day19")?;

    let grid = scan_grid(&bytes, 50, 50);

    let affected = grid.map.values()
        .filter(|&&v| v == 1)
        .count();

    println!("{:?}", affected);

    Ok(())
}

/*
fn dump(bytes: &[Word], topleft: &Coord, botright: &Coord) {
    for y in topleft.y-5..=botright.y {
        let mut seen_on = false;

        for x in topleft.x-5..=botright.x {
            let c = get_coord(bytes, &Coord { x, y });

            print!("{}", c);

            if c == 1 {
                seen_on = true;
            } else if seen_on {
                break;
            }
        }
        print!("\n");
    }
}
*/

fn square_topleft_from_botleft(bytes: &[Word], &Coord { x, y }: &Coord) -> Option<Coord> {
    let side = 99;
    let topleft = Coord { x, y: y - side };
    let topright = Coord { x: x + side, y: y - side };
    let botright = Coord { x: x + side, y };

    assert!(get_coord(bytes, &Coord { x, y }) == 1);

    if get_coord(bytes, &topleft) == 1
    && get_coord(bytes, &topright) == 1
    && get_coord(bytes, &botright) == 1 {
        //dump(bytes, &topleft, &botright);
        Some(topleft)
    } else {
        None
    }
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = bytes("./input-day19")?;

    // pick somewhere not in the tunnel
    let mut coord = Coord::new(0, 10);
    assert!(get_coord(&bytes, &coord) == 0);

    let topleft = loop {
        // get into the tunnel
        while get_coord(&bytes, &coord) == 0 {
            coord.x += 1;
        }

        //println!("trying {:?}", coord);

        match square_topleft_from_botleft(&bytes, &coord) {
            Some(topleft) => break topleft,
            None => {
                coord.y += 1;
                while get_coord(&bytes, &coord) == 1 {
                    coord.x -= 1;
                }
            },
        };
    };

    println!("topleft: {:?}", topleft);
    println!("answer: {}", topleft.x * 10000 + topleft.y);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //part1()?;
    part2()?;

    Ok(())
}
