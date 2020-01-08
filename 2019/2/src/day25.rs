use std::io;

mod parse;
mod lib;
mod ascii;

use itertools::Itertools;
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
    let initial_steps = [
        "north",
        "west",
        "west",
        "west",
        "west",
        "east",
        "south",
        "north",
        "west",
        "take tambourine",
        "east",
        "west",
        "east",
        "east",
        "take candy cane",
        "east",
        "east",
        "east",
        "take ornament",
        "north",
        "north",
        "take dark matter",
        "south",
        "south",
        "west",
        "south",
        "south",
        "east",
        "take whirled peas",
        "west",
        "north",
        "north",
        "west",
        "north",
        "take astrolabe",
        "east",
        "take hologram",
        "east",
        "take klein bottle",
        "west",
        "south",
        "west",
        "north",
    ];

    let bytes = parse::bytes("./input-day25")?;

    let mut machine = AsciiMachine::new(&bytes);
    let mut input = initial_steps
        .join("\n");
    input.push('\n');

    let _out = machine.run(input);

    let inventory = vec![
        "ornament",
        "klein bottle",
        "dark matter",
        "candy cane",
        "hologram",
        "astrolabe",
        "whirled peas",
        "tambourine",
    ];

    let mut can_continue = vec![];

    for n_to_drop in 1..inventory.len()-1 {
        for inv in inventory.iter().combinations(n_to_drop) {
            let mut machine = machine.clone();

            let mut input = inv
                .iter()
                .map(|&item| String::from("drop ") + item + "\n")
                .collect::<Vec<_>>();

            input.push("north\n".into());
            let input = input.join("");

            let output = machine.run(input);

            let success = !output.contains("ejected back to the checkpoint");
            if success {
                println!("permutation {:?}: {} {}", inv, success, output);
                can_continue.push(inv);
            }
        }
    }

    println!("valid inventories:");
    for valid in can_continue {
        println!("  {:?}", valid/*.iter().map(|&s| s)*/);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1()?;

    Ok(())
}
