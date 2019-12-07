pub type Word = i64; // may be signed

const OP_ADD: Word = 1; // *[3] = *[1] + *[2]
const OP_MUL: Word = 2; // *[3] = *[1] + *[2]
const OP_HALT: Word = 99; // no arg
const OP_INPUT: Word = 3; // output
const OP_OUTPUT: Word = 4; // input
const OP_JNZ: Word = 5; // input1, input2
const OP_JZ: Word = 6; // input1, input2
const OP_LT: Word = 7; // src,src,dest
const OP_EQ: Word = 8; // src,src,dest

fn decode_param(op: Word, iparam: u32) -> bool {
    let mut paramcodes = op / 100;

    for _ in 0 .. iparam {
        paramcodes /= 10;
    }

    paramcodes % 10 == 0
}

pub enum IntCodeState<'a> {
    Halted(Vec<Word>),
    Running {
        mem: &'a mut [Word],
        output: Vec<Word>,
        ip: usize,
    },
}

pub fn interpret_async<'a>(
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
                    inputs.remove(0)
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

pub fn interpret(
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

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_io(input: Word, expected_output: Word, memory: &[Word]) {
        let mut bytes = Vec::from(&memory[..]);
        let mut input = vec![input];

        let output = interpret(&mut bytes, &mut input);

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], expected_output);
    }

    #[test]
    fn test_decode_param() {
        assert_eq!(decode_param(1002, 0), true);
        assert_eq!(decode_param(1002, 1), false);
    }

    #[test]
    fn test_day2_part1_eg0() {
        let mut bytes = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_day2_part1_eg1() {
        let mut bytes = [1, 0, 0, 0, 99];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_day2_part1_eg2() {
        let mut bytes = [2, 3, 0, 3, 99];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_day2_part1_eg3() {
        let mut bytes = [2, 4, 4, 5, 99, 0];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_day2_part1_eg4() {
        let mut bytes = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_day5_part1_eg1() {
        let mut bytes = [1101,100,-1,4,0];
        //is a valid program (find 100 + -1, store the result in position 4)
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [1101,100,-1,4,100 + -1]);
    }

    #[test]
    fn test_day5_part1_eg2() {
        let mut bytes = [1002,4,3,4,33]; // exit after mul
        interpret(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [1002,4,3,4,99]);
    }

    #[test]
    fn test_day5_part2_eg1() {
        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let bytes = [3,9,8,9,10,9,4,9,99,-1,8];

        expect_io(8, 1, &bytes);
        expect_io(7, 0, &bytes);
        expect_io(9, 0, &bytes);
    }

    #[test]
    fn test_day5_part2_eg2() {
        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let bytes = [3,9,7,9,10,9,4,9,99,-1,8];

        expect_io(3, 1, &bytes);
        expect_io(8, 0, &bytes);
        expect_io(9, 0, &bytes);
    }

    #[test]
    fn test_day5_part2_eg3() {
        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let bytes = [3,3,1108,-1,8,3,4,3,99];

        expect_io(3, 0, &bytes);
        expect_io(8, 1, &bytes);
        expect_io(9, 0, &bytes);
    }

    #[test]
    fn test_day5_part2_eg4() {
        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let bytes = [3,3,1107,-1,8,3,4,3,99];

        expect_io(3, 1, &bytes);
        expect_io(8, 0, &bytes);
        expect_io(9, 0, &bytes);
    }

    #[test]
    fn test_day5_part2_eg5() {
        // !!input (using position addressing)
        let bytes_position = [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];

        expect_io(1, 1, &bytes_position);
        expect_io(0, 0, &bytes_position);
        expect_io(2, 1, &bytes_position);
        expect_io(50, 1, &bytes_position);

        // !!input (using immediate addressing)
        let bytes_immediate = [3,3,1105,-1,9,1101,0,0,12,4,12,99,1];

        expect_io(1, 1, &bytes_immediate);
        expect_io(0, 0, &bytes_immediate);
        expect_io(2, 1, &bytes_immediate);
        expect_io(50, 1, &bytes_immediate);
    }

    #[test]
    fn test_day5_part2_eg6() {
        // input < 8 ? 999 : input == 8 ? 1000 : 1001
        let bytes = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];

        // The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8

        expect_io(7, 999, &bytes);
        expect_io(8, 1000, &bytes);
        expect_io(9, 1001, &bytes);
    }
}
