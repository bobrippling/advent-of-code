use std::fs;

mod instruction;
use instruction::Instruction;

mod machine;
use machine::Machine;

type Result<T> = std::result::Result<T , Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let s = fs::read_to_string("./input.txt")?;
    let instructions = parse_instructions(&s)?;

    println!("Part 1: {}", part1(&instructions[..]));
    println!("Part 2: {}", part2(&instructions[..]));

    Ok(())
}

fn parse_instructions(s: &str) -> Result<Vec<Instruction>> {
    s
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            l
                .parse()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        })
        .collect::<Result<Vec<_>>>()
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut machine = Machine::new(instructions);

    run_machine_detecting_loop(&mut machine);

    machine.acc
}

fn run_machine_detecting_loop(machine: &mut Machine) {
    let mut ran = vec![false; machine.instructions.len()];

    machine.run(&mut |machine| {
        if ran[machine.ip] {
            return false;
        }

        ran[machine.ip] = true;
        return true;
    });
}

#[test]
fn test_part1() {
    let instructions = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let instructions = parse_instructions(&instructions).unwrap();

    let acc = part1(&instructions[..]);
    assert_eq!(acc, 5);
}

fn part2(instructions_slice: &[Instruction]) -> i32 {
    let mut instructions = Vec::new();
    instructions.extend_from_slice(instructions_slice);

    fn toggle_instruction(i: &mut Instruction) {
        match *i {
            Instruction::Jmp(x) => {
                *i = Instruction::Nop(x);
            },
            Instruction::Nop(x) => {
                *i = Instruction::Jmp(x);
            },
            Instruction::Acc(_) => {},
        };
    }

    for i in 0..instructions.len() {
        toggle_instruction(&mut instructions[i]);

        let mut machine = Machine::new(&instructions);
        run_machine_detecting_loop(&mut machine);

        if machine.halted() {
            return machine.acc;
        }
        // else, we detected the inf-loop, abort

        toggle_instruction(&mut instructions[i]);
    }

    panic!();
}

#[test]
fn test_part2() {
    let instructions = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let instructions = parse_instructions(&instructions).unwrap();

    let acc = part2(&instructions);
    assert_eq!(acc, 8);
}
