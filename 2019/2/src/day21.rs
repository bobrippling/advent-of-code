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
    let mut input = "".to_string();

    loop {
        let out = machine.run(input);
        print!("{}", out);

        if machine.is_running() {
            input = line();
        } else {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1()?;

    Ok(())
}
