use std::fs;

type Word = i64; // can be signed

const OP_ADD: Word = 1; // *[3] = *[1] + *[2]
const OP_MUL: Word = 2; // *[3] = *[1] + *[2]
const OP_HALT: Word = 99; // no arg
const OP_INPUT: Word = 3; // output
const OP_OUTPUT: Word = 4; // input
const OP_JNZ: Word = 5; // input1, input2
const OP_JZ: Word = 6; // input1, input2
const OP_LT: Word = 7; // src,src,dest
const OP_EQ: Word = 8; // src,src,dest

fn input(a: &mut Vec<Word>) -> Word {
    /*
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
    */
    a.remove(0)/*
    match a.remove(0) {
        Some(i) => i,
        None => panic!("no input")
    }*/
}

/*
fn output(w: Word) {
    println!("{}", w);
}
*/

fn decode_param(op: Word, iparam: u32) -> bool {
    let mut paramcodes = op / 100;

    for _ in 0 .. iparam {
        paramcodes /= 10;
    }

    paramcodes % 10 == 0
}

enum IntCodeState<'a> {
    Halted(Vec<Word>),
    Running {
        mem: &'a mut [Word],
        output: Vec<Word>,
        ip: usize,
    },
}

fn interpret_async<'a>(
    state: IntCodeState<'a>,
    inputs: &mut Vec<Word>
) -> IntCodeState<'a> {
    let ( bytes, mut output, mut i ) = match state {
        IntCodeState::Running { mem, output, ip } => (mem, output, ip),
        _ => panic!(),
    };

    loop {
        let op = bytes[i];
        let opmod = op % 100;

        match opmod {
            OP_ADD => {
                let (src1, src2, dest)
                    = (bytes[i + 1], bytes[i + 2], bytes[i + 3]);

                let (deref_1, deref_2) = (decode_param(op, 0), decode_param(op, 1));

                let i1 = if deref_1 { bytes[src1 as usize] } else { src1 };
                let i2 = if deref_2 { bytes[src2 as usize] } else { src2 };

                bytes[dest as usize] = (i1) .wrapping_add(i2);

                println!("{} + {} --> [{}]", i1, i2, dest as usize);

                i += 4;
            },

            OP_MUL => {
                let (src1, src2, dest)
                    = (bytes[i + 1], bytes[i + 2], bytes[i + 3]);

                let (deref_1, deref_2) = (decode_param(op, 0), decode_param(op, 1));

                let i1 = if deref_1 { bytes[src1 as usize] } else { src1 };
                let i2 = if deref_2 { bytes[src2 as usize] } else { src2 };

                bytes[dest as usize] = (i1) * (i2);

                println!("{} * {} --> [{}] (= {})", i1, i2, dest as usize, bytes[dest as usize]);

                i += 4;
            },

            OP_INPUT => {
                let dest = bytes[i + 1];

                let inp = if inputs.len() > 0 {
                    input(inputs)
                } else {
                    return IntCodeState::Running {
                        mem: bytes,
                        output,
                        ip: i,
                    };
                };

                bytes[dest as usize] = inp;

                println!("input {} --> [{}]", bytes[dest as usize], dest);

                i += 2;
            },

            OP_OUTPUT => {
                let src = bytes[i + 1];
                let deref = decode_param(op, 0);
                let o = if deref { bytes[src as usize] } else { src };

                println!("output {}", o);

                //output(o);
                output.push(o);

                i += 2;
            },

            OP_JNZ => {
                let (src1, src2) = (bytes[i + 1], bytes[i + 2]);
                let (deref_1, deref_2) = (decode_param(op, 0), decode_param(op, 1));
                let (src1_read, src2_read) = (
                    if deref_1 { bytes[src1 as usize] } else { src1 },
                    if deref_2 { bytes[src2 as usize] } else { src2 }
                );

                println!("jnz {} --> {}", src1_read, src2_read);
                if src1_read != 0 {
                    i = src2_read as usize;
                } else {
                    i += 3;
                }
            },

            OP_JZ => {
                let (src1, src2) = (bytes[i + 1 as usize], bytes[i + 2 as usize]);
                let (deref_1, deref_2) = (decode_param(op, 0), decode_param(op, 1));
                let (src1_read, src2_read) = (
                    if deref_1 { bytes[src1 as usize] } else { src1 },
                    if deref_2 { bytes[src2 as usize] } else { src2 }
                );

                println!("jz {} --> {}", src1_read, src2_read);
                if src1_read == 0 {
                    i = src2_read as usize;
                } else {
                    i += 3;
                }
            },

            OP_LT => {
                let (src1, src2, dest) = (bytes[i + 1 as usize], bytes[i + 2 as usize], bytes[i + 3 as usize]);
                let (deref_1, deref_2) = (decode_param(op, 0), decode_param(op, 1));
                let (src1_read, src2_read) = (
                    if deref_1 { bytes[src1 as usize] } else { src1 },
                    if deref_2 { bytes[src2 as usize] } else { src2 }
                );

                println!("{} < {} --> [{}]", src1_read, src2_read, dest);
                bytes[dest as usize] = if src1_read < src2_read { 1 } else { 0 };

                i += 4;
            },

            OP_EQ => {
                let (src1, src2, dest) = (bytes[i + 1 as usize], bytes[i + 2 as usize], bytes[i + 3 as usize]);
                let (deref_1, deref_2) = (decode_param(op, 0), decode_param(op, 1));
                let (src1_read, src2_read) = (
                    if deref_1 { bytes[src1 as usize] } else { src1 },
                    if deref_2 { bytes[src2 as usize] } else { src2 }
                );

                println!("{} == {} --> [{}]", src1_read, src2_read, dest);
                bytes[dest as usize] = if src1_read == src2_read { 1 } else { 0 };

                i += 4;
            },

            OP_HALT => {
                return IntCodeState::Halted(output);
            }

            _ => {
                eprintln!("unknown isn {}", opmod);
                panic!();
            },
        }
    }
}

fn interpret(
    bytes: &mut [Word],
    inputs: &mut Vec<Word>
) -> Vec<Word> {
    let state = IntCodeState::Running {
        mem: bytes,
        ip: 0,
        output: Default::default(),
    };
    let out = interpret_async(state, inputs);
    match out {
        IntCodeState::Halted(output) => output,
        _ => panic!(),
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

    interpret(&mut bytes, &mut Default::default());

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

                interpret(&mut bytes, &mut Default::default());

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

fn run_phase(phases: &Vec<Word>, bytes: &[Word]) -> Word {
    let mut last_i = 0;

    for phase in 0..=4 {
        let mut input = vec![phases[phase], last_i];
        let mut bytes = Vec::from(bytes);

        let output = interpret(&mut bytes, &mut input);
        assert_eq!(input.len(), 0);
        assert_eq!(output.len(), 1);
        let old = last_i;
        last_i = output[0];
        println!("  phase {:?}, input {} output {}",
                 phases[phase], old, last_i);
    }

    last_i
}

type Phase = Vec<Word>;

fn find_max_phase(bytes: &[Word]) -> (Word, Phase) {
    let mut max = 0;
    let mut max_phase = Vec::new();

    for a in 0 ..= 4 {
        for b in 0 ..= 4 {
            for c in 0 ..= 4 {
                for d in 0 ..= 4 {
                    for e in 0 ..= 4 {
                        if a == b || a == c || a == d || a == e {
                            continue;
                        }
                        if b == c || b == d || b == e {
                            continue;
                        }
                        if c == d || c == e {
                            continue;
                        }
                        if d == e {
                            continue;
                        }

                        let phase = vec![a, b, c, d, e];
                        println!("phase: {:?}", phase);
                        let m = run_phase(&phase, bytes);
                        println!("  max: {}", m);
                        if m >= max {
                            max = m;
                            max_phase = phase;
                        }
                    }
                }
            }
        }
    }

    (max, max_phase)
}

fn run_phase_feedback(phases: &Vec<Word>, bytes: &[Word]) -> Word {
    #[derive(PartialEq)]
    enum State {
        Init,
        WantInput,
        Halted,
    }
    struct Amplifier {
        code: Vec<Word>,
        state: State,
        input_queue: Vec<Word>,
        output_queue: Vec<Word>,
        ip: usize,
    };

    impl Amplifier {
        fn new(phase: Word, code: &[Word]) -> Self {
            Self {
                code: Vec::from(code),
                state: State::Init,
                input_queue: vec![phase],
                output_queue: vec![],
                ip: 0,
            }
        }

        fn run(&mut self) {
            match self.state {
                State::Init => {
                    let ics = IntCodeState::Running {
                        mem: &mut self.code,
                        output: Default::default(),
                        ip: 0,
                    };
                    let ics = interpret_async(ics, &mut self.input_queue);
                    match ics {
                        IntCodeState::Halted(mut output) => {
                            self.state = State::Halted;
                            self.output_queue.append(&mut output);
                        },
                        IntCodeState::Running {
                            mem: _, mut output, ip,
                        } => {
                            self.state = State::WantInput;
                            self.ip = ip;
                            self.output_queue.append(&mut output);
                        }
                    };
                },
                State::WantInput => {
                    let ics = IntCodeState::Running {
                        mem: &mut self.code,
                        output: Default::default(),
                        ip: self.ip,
                    };
                    let ics = interpret_async(ics, &mut self.input_queue);
                    match ics {
                        IntCodeState::Halted(mut output) => {
                            self.state = State::Halted;
                            self.output_queue.append(&mut output);
                        },
                        IntCodeState::Running {
                            mem: _, mut output, ip,
                        } => {
                            self.state = State::WantInput;
                            self.ip = ip;
                            self.output_queue.append(&mut output);
                        }
                    };
                },
                State::Halted => {
                },
            };
        }
    }

    let mut amplifiers = [
        Amplifier::new(phases[0], From::from(bytes)),
        Amplifier::new(phases[1], From::from(bytes)),
        Amplifier::new(phases[2], From::from(bytes)),
        Amplifier::new(phases[3], From::from(bytes)),
        Amplifier::new(phases[4], From::from(bytes)),
    ];

    amplifiers[0].input_queue.push(0);

    let mut output_for_next = Vec::new();
    loop {
        let mut foundrunning = false;

        for amp in amplifiers.iter_mut() {
            if output_for_next.len() > 0 {
                amp.input_queue.append(&mut output_for_next);
            }
            if amp.output_queue.len() > 0 {
                output_for_next.append(&mut amp.output_queue);
            }

            amp.run();
            if amp.state != State::Halted {
                foundrunning = true;
            }
        }

        if !foundrunning {
            break;
        }
    }

    amplifiers[amplifiers.len()-1].output_queue[0]
}

fn find_max_phase_feedback(bytes: &[Word]) -> (Word, Vec<Word>) {
    let mut max = 0;
    let mut max_phase = Vec::new();

    for a in 5 ..= 9 {
        for b in 5 ..= 9 {
            for c in 5 ..= 9 {
                for d in 5 ..= 9 {
                    for e in 5 ..= 9 {
                        if a == b || a == c || a == d || a == e {
                            continue;
                        }
                        if b == c || b == d || b == e {
                            continue;
                        }
                        if c == d || c == e {
                            continue;
                        }
                        if d == e {
                            continue;
                        }

                        let phase = vec![a, b, c, d, e];
                        println!("phase: {:?}", phase);
                        let m = run_phase_feedback(&phase, bytes);
                        println!("  max: {}", m);
                        if m >= max {
                            max = m;
                            max_phase = phase;
                        }
                    }
                }
            }
        }
    }

    (max, max_phase)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input")?;
    let bytes = s
        .trim_end()
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<Word>, _>>()?;

    //part1(&bytes);
    //part2(&bytes);

    //interpret(&mut bytes, &mut Default::default());
    //show_bytes(&bytes);

    //let (max, phase) = find_max_phase(&bytes);
    //println!("max={} phasse={:?}", max, phase);

    let (max, phase) = find_max_phase_feedback(&bytes);
    println!("max={} phasse={:?}", max, phase);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_param() {
        assert_eq!(decode_param(1002, 0), true);
        assert_eq!(decode_param(1002, 1), false);
    }

    #[test]
    fn test_eg1() {
        let mut bytes = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_eg2() {
        let mut bytes = [1, 0, 0, 0, 99];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_eg3() {
        let mut bytes = [2, 3, 0, 3, 99];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_eg4() {
        let mut bytes = [2, 4, 4, 5, 99, 0];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_eg5() {
        let mut bytes = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_eg6() {
        let mut bytes = [1101,100,-1,4,0];
        //is a valid program (find 100 + -1, store the result in position 4)
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [1101,100,-1,4,100 + -1]);
    }

    #[test]
    fn test_eg7() {
        let mut bytes = [1002,4,3,4,33]; // exit after mul
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [1002,4,3,4,99]);
    }

    /*#[test]
    fn test_eg8() {
     3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9
     3,3,1105,-1,9,1101,0,0,12,4,12,99,1
    }*/
//     3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
//     3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
//     3,3,1108,-1,8,3,4,3,99   - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
//     3,3,1107,-1,8,3,4,3,99   - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).

    #[test]
    fn test_run_phase() {
        let mut bytes = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];

        let output = interpret(
            &mut bytes,
            &mut vec![4, 0]
        );

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 4);
    }

    #[test]
        fn test_eg9() {
            let expected_max = 43210;
            let expected_phase = [4,3,2,1,0];
            let bytes = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];

            let (max, phase) = find_max_phase(&bytes);
            assert_eq!(phase, expected_phase);
            assert_eq!(max, expected_max);
        }

     #[test]
    fn test_eg10() {
        let expected_max = 54321;
        let expected_phase = [0,1,2,3,4];
        let mut bytes = [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];

            let (max, phase) = find_max_phase(&mut bytes);
            assert_eq!(max, expected_max);
            assert_eq!(phase, expected_phase);
    }

     #[test]
    fn test_eg11() {
        let expected_max = 65210;
        let expected_phase = [1,0,4,3,2];
        let mut bytes = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];

            let (max, phase) = find_max_phase(&mut bytes);
            assert_eq!(max, expected_max);
            assert_eq!(phase, expected_phase);
    }

    // -----

    #[test]
    fn test_eg12() {
        let expected_max =139629729;
        let expected_phase = [9,8,7,6,5];
        let bytes = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26, 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        let (max, phase) = find_max_phase_feedback(&bytes);

        assert_eq!(max, expected_max);
        assert_eq!(phase, expected_phase);
    }

    #[test]
    fn test_eg13() {
        let expected_max =18216;
        let expected_phase = [9,7,8,5,6];
        let bytes = [ 3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10 ];

        let (max, phase) = find_max_phase_feedback(&bytes);

        assert_eq!(max, expected_max);
        assert_eq!(phase, expected_phase);
    }

}
