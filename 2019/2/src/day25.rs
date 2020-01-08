use std::io;

mod parse;
mod lib;
mod ascii;

use ascii::AsciiMachine;

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
    let bytes = parse::bytes("./input-day25")?;

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
