use std::io;

/*
macro_rules! dist2reg {
    (1) => {"A"};
    (2) => {"B"};
    (3) => {"C"};
    (4) => {"D"};
    (5) => {"E"};
    (6) => {"F"};
    (7) => {"G"};
    (8) => {"H"};
    (9) => {"I"};
    ($n:expr) => {
        ["A", "B", "C", "D", "E", "F", "G", "H", "I"][$n]
    };
}

macro_rules! gap_at {
    ($n: literal) => {
        concat!("NOT ", $n, " T") //, "OR T, J"
    }
}

macro_rules! tile_at {
    ($n: literal) => {
        concat!("AND ", dist2reg!($n), " J")
    }
}
*/

mod parse;
mod lib;
mod ascii;

use ascii::AsciiMachine;

enum Isn {
    And { x: bool, y: bool }, // y = y & x
    Or  { x: bool, y: bool }, // y = y | x
    Not { x: bool, y: bool }, // y = !x
    // y must be register J or T
    // x can be J, T, or ABCD
}

struct SpringDroid {
    isns: Vec<Isn>, // 15 max
    reg_t: bool, // temp
    reg_j: bool, // jump
    reg_a: bool, // one tile away is ground
    reg_b: bool, // two tiles away is ground
    reg_c: bool, // three tiles away is ground
    reg_d: bool, // four tiles away is ground
}

fn line() -> String {
    let mut line = String::new();

    match io::stdin().read_line(&mut line) {
        Ok(_) => {
            line
        },
        Err(e) => {
            eprintln!("error reading stdin: {}", e);
            std::process::exit(1);
        }
    }
}

fn n2reg(n: usize) -> &'static str {
    match n {
        1 => "A",
        2 => "B",
        3 => "C",
        4 => "D",
        5 => "E",
        6 => "F",
        7 => "G",
        8 => "H",
        9 => "I",
        _ => panic!(),
    }
}

fn gap_at(n: usize) -> String {
    format!("NOT {} T\nOR T J\n", n2reg(n)).into()
}

fn tile_at(n: usize) -> String {
    format!("AND {} J\n", n2reg(n)).into()
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = parse::bytes("./input-day21")?;

    let mut machine = AsciiMachine::new(&bytes);
    let input = [
        // jump if: gap @ 1, 2 or 3 and not at 4
        //"NOT A T", "OR T J",
        gap_at(1),

        //"NOT B T", "OR T J",
        gap_at(2),

        //"NOT C T", "OR T J",
        gap_at(3),

        //"AND D J",
        tile_at(4),

        "WALK\n".into(),
    ]
        .iter()
        .map(|s| &s[..])
        .collect::<Vec<&str>>()
        .join("");

    println!("input:\n{}", input);

    let mut out = machine.run_intcode_output(input);
    let last = out.pop().unwrap();
    let s = ascii::to_string(&out);

    println!("{}", s);
    println!("final word: {}", last);

    assert!(!machine.is_running());

    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = parse::bytes("./input-day21")?;

    let mut machine = AsciiMachine::new(&bytes);
    let mut input = [
        // jump if:
        //   gap @ 1, 2 or 3 and not at 4 (first jump will succeed)
        // AND
        //   tile @ 5 and tile at 9
        //
        gap_at(1),
        gap_at(2),
        gap_at(3),

        tile_at(5),
        tile_at(9),
        //"NOT A T", "OR T J",

        //"AND D J",

        "RUN\n".into(),
    ]
        .iter()
        .map(|s| &s[..])
        .collect::<Vec<&str>>()
        .join("");

    loop {
        println!("inut: {}", input);
        let mut out = machine.run(input);

        println!("{}", out);

        if !machine.is_running() {
            break;
        }

        input = line();
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //part1()?;
    part2()?;
    //println!("{}", tile_at!(2));

    Ok(())
}
