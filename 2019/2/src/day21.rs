use std::io;

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

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = parse::bytes("./input-day21")?;

    let mut machine = AsciiMachine::new(&bytes);
    let input = [
        // jump if: gap @ 1, 2 or 3 and not at 4
        "NOT A T",
        "OR T J",

        "NOT B T",
        "OR T J",

        "NOT C T",
        "OR T J",

        "AND D J",

        "WALK",
    ]
        .iter()
        .map(|&s| String::from(s) + "\n")
        .collect::<Vec<_>>()
        .join("");

    let mut out = machine.run_intcode_output(input);
    let last = out.pop().unwrap();
    let s = ascii::to_string(&out);

    println!("{}", s);
    println!("final word: {}", last);

    assert!(!machine.is_running());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1()?;

    Ok(())
}
