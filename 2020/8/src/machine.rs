use super::instruction::Instruction;

pub struct Machine<'a> {
    pub acc: i32,
    pub ip: usize,
    pub instructions: &'a [Instruction],
}

impl<'a> Machine<'a> {
    pub fn new(instructions: &'a [Instruction]) -> Self {
        Machine {
            acc: 0,
            ip: 0,
            instructions,
        }
    }
}

impl Machine<'_> {
    pub fn run(&mut self, pre_step: &mut dyn FnMut(&Machine) -> bool) {
        loop {
            if self.halted() {
                break;
            }

            if !pre_step(self) {
                break;
            }

            self.step();
        }
    }

    pub fn halted(&self) -> bool {
        self.ip == self.instructions.len()
    }
}

impl Machine<'_> {
    fn step(&mut self) {
        let i = &self.instructions[self.ip];

        match i {
            Instruction::Acc(a) => {
                self.acc += *a;
                self.ip += 1;
            },
            Instruction::Jmp(j) => {
                let j = *j;

                self.ip = (self.ip as isize + j as isize) as usize;
            },
            Instruction::Nop(_n) => {
                self.ip += 1;
            },
        };
    }
}
