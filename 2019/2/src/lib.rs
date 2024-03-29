#![allow(dead_code)]

pub type Word = i64; // may be signed

const OP_ADD: Word = 1; // *[3] = *[1] + *[2]
const OP_MUL: Word = 2; // *[3] = *[1] + *[2]
const OP_HALT: Word = 99; // no arg
const OP_INPUT: Word = 3; // [1] --> output
const OP_OUTPUT: Word = 4; // input --> [1]
const OP_JNZ: Word = 5; // [1] && jmp [2]
const OP_JZ: Word = 6; // [1] == 0 && jmp [2]
const OP_LT: Word = 7; // [1] < [2] --> [3]
const OP_EQ: Word = 8; // [1] == [2] --> [3]
const OP_RELATIVE_BASE: Word = 9; // [1] == [2] --> [3]

macro_rules! debug {
    ( $self: ident, $fmt: literal) => {
        if $self.debug {
            eprintln!($fmt);
        }
    };
    ( $self: ident, $fmt: literal, $arg: expr) => {
        if $self.debug {
            eprintln!($fmt, $arg);
        }
    };
    ( $self: ident, $fmt: literal, $($args: expr),*) => {
        if $self.debug {
            eprintln!($fmt, $($args),*);
        }
    };
}

#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Running,
    Halted,
}

#[derive(Clone)]
pub struct IntCodeMachine {
    state: State,

    mem: Vec<Word>,
    ip: usize,
    relative_base: Word,

    debug: bool,
}

#[derive(Debug)]
enum Operand {
    Position(Word),
    Immediate(Word),
    RelativeBase(Word),
}

fn operand_mode(op: Word, iparam: usize) -> Word {
    let mut paramcodes = op / 100;

    for _ in 0 .. iparam {
        paramcodes /= 10;
    }

    paramcodes % 10
}

fn decode_opcode(op: Word) -> Word {
    op % 100
}

impl IntCodeMachine {
    pub fn new(mem: &[Word], debug: bool) -> Self {
        Self {
            state: State::Running,
            mem: From::from(mem),
            ip: 0,
            relative_base: 0,
            debug,
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn is_running(&self) -> bool {
        match self.state() {
            State::Running => true,
            State::Halted => false,
        }
    }

    pub fn memory(&self) -> &[Word] {
        &self.mem
    }

    pub fn load_memory(&mut self, mem: &[Word]) {
        self.mem.clear();
        self.mem.extend_from_slice(mem);
    }

    fn memref(&mut self, index: usize) -> &mut Word {
        if index >= self.mem.len() {
            self.mem.resize(index + 1, 0);
        }
        &mut self.mem[index]
    }

    fn mem(&mut self, index: usize) -> Word {
        *self.memref(index)
    }

    fn decode_operand(&mut self, index: usize) -> Operand {
        let op = self.mem(self.ip);
        let operand = self.mem(self.ip + 1 + index);

        match operand_mode(op, index) {
            0 => Operand::Position(operand),
            1 => Operand::Immediate(operand),
            2 => Operand::RelativeBase(operand),
            _ => panic!(),
        }
    }

    fn operand_input(&mut self, index: usize) -> Word {
        match self.decode_operand(index) {
            Operand::Position(pos) => self.mem(pos as usize),
            Operand::Immediate(val) => val,
            Operand::RelativeBase(val) => self.mem((self.relative_base + val) as usize),
        }
    }

    fn operand_output(&mut self, index: usize) -> (&mut Word, Word) {
        let op = self.decode_operand(index);
        let pos = match op {
            Operand::Position(w) => w,
            Operand::Immediate(_) => panic!("can't output to {:?}", op),
            Operand::RelativeBase(w) => self.relative_base + w,
        };

        (self.memref(pos as usize), pos)
    }

    /*
    fn dump(&self) {
        for b in &self.mem {
            eprint!("{},", b);
        }
        eprintln!("");
    }
    */

    pub fn interpret_async(
        &mut self,
        inputs: &mut Vec<Word>
    ) -> Vec<Word> {
        match self.state {
            State::Running => {},
            State::Halted => panic!(),
        }

        let mut output = Vec::new();

        loop {
            let isn = self.mem(self.ip);

            match decode_opcode(isn) {
                OP_ADD => {
                    let (lhs, rhs) = (
                        self.operand_input(0),
                        self.operand_input(1),
                    );

                    let (dest, dest_i) = self.operand_output(2);
                    *dest = lhs + rhs;

                    debug!(self, "{} + {} --> [{}]", lhs, rhs, dest_i);

                    self.ip += 4;
                },

                OP_MUL => {
                    let (lhs, rhs) = (
                        self.operand_input(0),
                        self.operand_input(1),
                    );

                    let (dest, dest_i) = self.operand_output(2);
                    *dest = lhs * rhs;

                    debug!(self, "{} * {} --> [{}]", lhs, rhs, dest_i);

                    self.ip += 4;
                },

                OP_INPUT => {
                    let (dest, dest_i) = self.operand_output(0);

                    if inputs.len() == 0 {
                        break;
                    };

                    let input = inputs.remove(0);
                    *dest = input;

                    debug!(self, "input {} --> [{}]", input, dest_i);

                    self.ip += 2;
                },

                OP_OUTPUT => {
                    let src = self.operand_input(0);

                    debug!(self, "output {}", src);

                    output.push(src);

                    self.ip += 2;
                },

                OP_JNZ => {
                    let (to_test, target) = (
                        self.operand_input(0),
                        self.operand_input(1),
                    );

                    debug!(self, "jnz {} --> {}", to_test, target);

                    if to_test != 0 {
                        self.ip = target as usize;
                    } else {
                        self.ip += 3;
                    }
                },

                OP_JZ => {
                    let (to_test, target) = (
                        self.operand_input(0),
                        self.operand_input(1),
                    );

                    debug!(self, "jz {} --> {}", to_test, target);

                    if to_test == 0 {
                        self.ip = target as usize;
                    } else {
                        self.ip += 3;
                    }
                },

                OP_LT => {
                    let (lhs, rhs) = (
                        self.operand_input(0),
                        self.operand_input(1),
                    );

                    let (dest, dest_i) = self.operand_output(2);
                    *dest = (lhs < rhs) as _;

                    debug!(self, "{} < {} --> [{}]", lhs, rhs, dest_i);

                    self.ip += 4;
                },

                OP_EQ => {
                    let (lhs, rhs) = (
                        self.operand_input(0),
                        self.operand_input(1),
                    );

                    let (dest, dest_i) = self.operand_output(2);
                    *dest = (lhs == rhs) as _;

                    debug!(self, "{} == {} --> [{}]", lhs, rhs, dest_i);

                    self.ip += 4;
                },

                OP_RELATIVE_BASE => {
                    let operand = self.operand_input(0);

                    self.relative_base += operand;

                    debug!(self, "relative base <-- {} (operand {})", self.relative_base, operand);

                    self.ip += 2;
                },

                OP_HALT => {
                    debug!(self, "halt");
                    self.state = State::Halted;
                    break;
                }

                _ => {
                    panic!("unknown isn {}", isn);
                },
            }
        }

        output
    }
}

#[cfg(test)]
pub fn interpret_oneshot_mutmem(
    mem: &mut [Word],
    inputs: &mut Vec<Word>,
) -> Vec<Word> {
    let mut machine = IntCodeMachine::new(mem, false);

    let output = machine.interpret_async(inputs);

    assert_eq!(mem.len(), machine.mem.len());
    for i in 0..mem.len() {
        mem[i] = machine.mem[i];
    }

    match machine.state {
        State::Running => panic!("oneshot failed to complete"),
        State::Halted => output,
    }
}

#[cfg(test)]
pub fn interpret_oneshot(
    mem: &[Word],
    inputs: &mut Vec<Word>,
) -> Vec<Word> {
    let mut machine = IntCodeMachine::new(mem, false);

    let output = machine.interpret_async(inputs);

    match machine.state {
        State::Running => panic!("oneshot failed to complete"),
        State::Halted => output,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_io(input: Word, expected_output: Word, memory: &[Word]) {
        let mut input = vec![input];

        let output = interpret_oneshot(memory, &mut input);

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], expected_output);
    }

    #[test]
    fn test_day5_addressing() {
        assert_eq!(operand_mode(1002, 0), 0);
        assert_eq!(operand_mode(1002, 1), 1);

        assert_eq!(operand_mode(0202, 0), 2);
        assert_eq!(operand_mode(0202, 1), 0);

        assert_eq!(operand_mode(1202, 0), 2);
        assert_eq!(operand_mode(1202, 1), 1);
    }

    #[test]
    fn test_day2_part1_eg0() {
        let mut bytes = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        interpret_oneshot_mutmem(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_day2_part1_eg1() {
        let mut bytes = [1, 0, 0, 0, 99];
        interpret_oneshot_mutmem(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_day2_part1_eg2() {
        let mut bytes = [2, 3, 0, 3, 99];
        interpret_oneshot_mutmem(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_day2_part1_eg3() {
        let mut bytes = [2, 4, 4, 5, 99, 0];
        interpret_oneshot_mutmem(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_day2_part1_eg4() {
        let mut bytes = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        interpret_oneshot_mutmem(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_day5_part1_eg1() {
        let mut bytes = [1101,100,-1,4,0];
        //is a valid program (find 100 + -1, store the result in position 4)
        interpret_oneshot_mutmem(&mut bytes, &mut Default::default());
        assert_eq!(bytes, [1101,100,-1,4,100 + -1]);
    }

    #[test]
    fn test_day5_part1_eg2() {
        let mut bytes = [1002,4,3,4,33]; // exit after mul
        interpret_oneshot_mutmem(&mut bytes, &mut Default::default());
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

    #[test]
    fn test_day9_relative_base() {
        let mem = [
            109,3, // relative base: 3
            109,4, // relative base: 7
            204,-5, // output value at address 2 (109)
            99,
        ];

        let output = interpret_oneshot(&mem, &mut Default::default());

        assert_eq!(output, vec![109]);
    }

    #[test]
    fn test_day9_oob() {
        let mem = [
            4,20, // output value at address 20 (0)
            99,
        ];

        let output = interpret_oneshot(&mem, &mut Default::default());

        assert_eq!(output, vec![0]);
    }

    #[test]
    fn test_day9_eg1() {
        let mem = [
            109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99
        ];

        let output = interpret_oneshot(&mem, &mut Default::default());

        assert_eq!(output, mem);
    }

    #[test]
    fn test_day9_eg2() {
        let mem = [
            1102,34915192,34915192,7,4,7,99,0
        ];

        let output = interpret_oneshot(&mem, &mut Default::default());

        assert_eq!(output, vec![1219070632396864]);
    }

    #[test]
    fn test_day9_eg3() {
        let mem = [
            104,1125899906842624,99
        ];

        let output = interpret_oneshot(&mem, &mut Default::default());

        assert_eq!(output, vec![1125899906842624]);
    }
}
