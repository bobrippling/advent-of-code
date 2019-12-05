// You notify the Elves that the computer's magic smoke seems to have escaped. "That computer ran Intcode programs like the gravity assist program it was working on; surely there are enough spare parts up there to build a new Intcode computer!"
//
// An Intcode program is a list of integers separated by commas (like 1,0,0,3,99). To run one, start by looking at the first integer (called position 0). Here, you will find an opcode - either 1, 2, or 99. The opcode indicates what to do; for example, 99 means that the program is finished and should immediately halt. Encountering an unknown opcode means something went wrong.
//
// Opcode 1 adds together numbers read from two positions and stores the result in a third position. The three integers immediately after the opcode tell you these three positions - the first two indicate the positions from which you should read the input values, and the third indicates the position at which the output should be stored.
//
// For example, if your Intcode computer encounters 1,10,20,30, it should read the values at positions 10 and 20, add those values, and then overwrite the value at position 30 with their sum.
//
// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them. Again, the three integers after the opcode indicate where the inputs and outputs are, not their values.
//
// Once you're done processing an opcode, move to the next one by stepping forward 4 positions.
//
// For example, suppose you have the following program:
//
// 1,9,10,3,2,3,11,0,99,30,40,50
//
// For the purposes of illustration, here is the same program split into multiple lines:
//
// 1,9,10,3,
// 2,3,11,0,
// 99,
// 30,40,50
//
// The first four integers, 1,9,10,3, are at positions 0, 1, 2, and 3. Together, they represent the first opcode (1, addition), the positions of the two inputs (9 and 10), and the position of the output (3). To handle this opcode, you first need to get the values at the input positions: position 9 contains 30, and position 10 contains 40. Add these numbers together to get 70. Then, store this value at the output position; here, the output position (3) is at position 3, so it overwrites itself. Afterward, the program looks like this:
//
// 1,9,10,70,
// 2,3,11,0,
// 99,
// 30,40,50
//
// Step forward 4 positions to reach the next opcode, 2. This opcode works just like the previous, but it multiplies instead of adding. The inputs are at positions 3 and 11; these positions contain 70 and 50 respectively. Multiplying these produces 3500; this is stored at position 0:
//
// 3500,9,10,70,
// 2,3,11,0,
// 99,
// 30,40,50
//
// Stepping forward 4 more positions arrives at opcode 99, halting the program.
//
// Here are the initial and final states of a few more small programs:
//
//     1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
//     2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
//     2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
//     1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
//
// Once you have a working computer, the first step is to restore the gravity assist program (your puzzle input) to the "1202 program alarm" state it had just before the last computer caught fire. To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?

// part 2
// "Good, the new computer seems to be working correctly! Keep it nearby during this mission - you'll probably use it again. Real Intcode computers support many more features than your new one, but we'll let you know what they are as you need them."
//
// "However, your current priority should be to complete your gravity assist around the Moon. For this mission to succeed, we should settle on some terminology for the parts you've already built."
//
// Intcode programs are given as a list of integers; these values are used as the initial state for the computer's memory. When you run an Intcode program, make sure to start by initializing memory to the program's values. A position in memory is called an address (for example, the first value in memory is at "address 0").
//
// Opcodes (like 1, 2, or 99) mark the beginning of an instruction. The values used immediately after an opcode, if any, are called the instruction's parameters. For example, in the instruction 1,2,3,4, 1 is the opcode; 2, 3, and 4 are the parameters. The instruction 99 contains only an opcode and has no parameters.
//
// The address of the current instruction is called the instruction pointer; it starts at 0. After an instruction finishes, the instruction pointer increases by the number of values in the instruction; until you add more instructions to the computer, this is always 4 (1 opcode + 3 parameters) for the add and multiply instructions. (The halt instruction would increase the instruction pointer by 1, but it halts the program instead.)
//
// "With terminology out of the way, we're ready to proceed. To complete the gravity assist, you need to determine what pair of inputs produces the output 19690720."
//
// The inputs should still be provided to the program by replacing the values at addresses 1 and 2, just like before. In this program, the value placed in address 1 is called the noun, and the value placed in address 2 is called the verb. Each of the two input values will be between 0 and 99, inclusive.
//
// Once the program has halted, its output is available at address 0, also just like before. Each time you try a pair of inputs, make sure you first reset the computer's memory to the values in the program (your puzzle input) - in other words, don't reuse memory from a previous attempt.
//
// Find the input noun and verb that cause the program to produce the output 19690720. What is 100 * noun + verb? (For example, if noun=12 and verb=2, the answer would be 1202.)


////////////////

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
// Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.

use std::fs;

type Word = i64; // can be signed

const OP_ADD: Word = 1; // *[3] = *[1] + *[2]
const OP_MUL: Word = 2; // *[3] = *[1] + *[2]
const OP_HALT: Word = 99; // no arg
const OP_INPUT: Word = 3; // output
const OP_OUTPUT: Word = 4; // input

fn input() -> Word {
    use std::io;
    eprintln!("input");
    let mut line = String::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("error reading stdin: {}", e);
                std::process::exit(1);
            }
        };

        let line = line.trim_end();

        match line.parse::<Word>() {
            Ok(i) => return i,
            Err(_) => {
                eprintln!("couldn't parse {}, try again", line);
            }
        }
    }
}

fn output(w: Word) {
    println!("{}", w);
}

fn decode_param(op: Word, iparam: u32) -> bool {
    let mut paramcodes = op / 100;

    for _ in 0 .. iparam {
        paramcodes /= 10;
    }

    paramcodes % 10 == 0
}

fn interpret(bytes: &mut [Word]) {
    let mut i = 0;

    loop {
        let op = bytes[i];
        let opmod = op % 100;

        match opmod {
            OP_ADD => {
                let (src1, src2, dest)
                    = (bytes[i + 1], bytes[i + 2], bytes[i + 3]);

                let (deref_1, deref_2) = (decode_param(op, 0), decode_param(op, 1));

                bytes[dest as usize] = (if deref_1 { bytes[src1 as usize] } else { src1 })
                    .wrapping_add(if deref_2 { bytes[src2 as usize] } else { src2 });

                i += 4;
            },

            OP_MUL => {
                let (src1, src2, dest)
                    = (bytes[i + 1], bytes[i + 2], bytes[i + 3]);

                let (deref_1, deref_2) = (decode_param(op, 0), decode_param(op, 1));

                bytes[dest as usize] = (if deref_1 { bytes[src1 as usize] } else { src1 })
                    .wrapping_mul(if deref_2 { bytes[src2 as usize] } else { src2 });

                i += 4;
            },

            OP_INPUT => {
                let dest = bytes[i + 1];

                bytes[dest as usize] = input();

                i += 2;
            },

            OP_OUTPUT => {
                let src = bytes[i + 1];
                let deref = decode_param(op, 0);

                output(if deref { bytes[src as usize] } else { src });

                i += 2;
            },

            OP_HALT => {
                break;
            }

            _ => {
                eprintln!("unknown isn {}", opmod);
                panic!();
            },
        }
    }
}

fn show_bytes(bytes: &[Word]) {
    for b in bytes {
        print!("{},", b);
    }
    println!("");
}

#[allow(dead_code)]
fn part1(bytes_slice: &[Word]) {
    let mut bytes = Vec::new();
    bytes.resize(bytes_slice.len(), 0);
    bytes.copy_from_slice(bytes_slice);

    bytes[1] = 12;
    bytes[2] = 2;

    println!("input");
    show_bytes(&bytes);

    interpret(&mut bytes);

    println!("output");
    show_bytes(&bytes);
}

#[allow(dead_code)]
fn part2(bytes_slice: &[Word]) {
    fn find(bytes_slice: &[Word]) -> Option<Word> {
        let mut bytes = Vec::new();
        bytes.resize(bytes_slice.len(), 0);

        let desired = 19690720;

        for noun in 0..=99 {
            for verb in 0..=99 {
                bytes.copy_from_slice(bytes_slice);
                bytes[1] = noun;
                bytes[2] = verb;

                interpret(&mut bytes);

                let output = bytes[0];

                //println!("{} and {} give {}", noun, verb, output);

                if output == desired {
                    return Some(100 * noun + verb);
                }
            }
        }

        None
    }

    match find(bytes_slice) {
        Some(answer) => println!("found: {}", answer),
        None => println!("Couldn't find match"),
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input")?;
    let mut bytes = s
        .trim_end()
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<Word>, _>>()?;

    //part1(&bytes);
    //part2(&bytes);

    interpret(&mut bytes);
    show_bytes(&bytes);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eg1() {
        let mut bytes = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        interpret(&mut bytes);
        assert_eq!(bytes, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_eg2() {
        let mut bytes = [1, 0, 0, 0, 99];
        interpret(&mut bytes);
        assert_eq!(bytes, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_eg3() {
        let mut bytes = [2, 3, 0, 3, 99];
        interpret(&mut bytes);
        assert_eq!(bytes, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_eg4() {
        let mut bytes = [2, 4, 4, 5, 99, 0];
        interpret(&mut bytes);
        assert_eq!(bytes, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_eg5() {
        let mut bytes = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        interpret(&mut bytes);
        assert_eq!(bytes, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_eg6() {
        let mut bytes = [1101,100,-1,4,0];
        //is a valid program (find 100 + -1, store the result in position 4)
        interpret(&mut bytes);
        assert_eq!(bytes, [1101,100,-1,4,100 + -1]);
    }

    #[test]
    fn test_eg7() {
        let mut bytes = [1002,4,3,4,33]; // exit after mul
        interpret(&mut bytes);
        assert_eq!(bytes, [1002,4,3,4,99]);
    }
}
