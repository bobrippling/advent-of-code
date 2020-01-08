use crate::lib::{IntCodeMachine, Word};

#[derive(Clone)]
pub struct AsciiMachine {
    icm: IntCodeMachine,
}

impl AsciiMachine {
    pub fn new(bytes: &[Word]) -> Self {
        Self {
            icm: IntCodeMachine::new(bytes, false),
        }
    }

    pub fn is_running(&self) -> bool {
        self.icm.is_running()
    }

    pub fn run_intcode_output(&mut self, input: String) -> Vec<Word> {
        let mut inv: Vec<Word> = input
            .chars()
            .map(|c| c as _)
            .collect();

        self.icm.interpret_async(&mut inv)
    }

    pub fn run(&mut self, input: String) -> String {
        to_string(&self.run_intcode_output(input))
    }
}

pub fn to_string(ents: &[Word]) -> String {
    ents
        .iter()
        .map(|&w| w as u8 as char)
        .collect()
}
