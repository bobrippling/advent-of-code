use crate::lib::{IntCodeMachine, Word};

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

    pub fn run(&mut self, input: String) -> String {
        let mut inv: Vec<Word> = input
            .chars()
            .map(|c| c as _)
            .collect();

        let out: Vec<Word> = self.icm.interpret_async(&mut inv);

        out
            .into_iter()
            .map(|w| w as u8 as char)
            .collect()
    }
}
