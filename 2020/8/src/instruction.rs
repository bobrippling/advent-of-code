#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
pub enum ParseErr {
    TooShort,
    InvalidNum,
    InvalidInstruction,
}

impl std::str::FromStr for Instruction {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Instruction, Self::Err> {
        if s.len() < 6 {
            return Err(ParseErr::TooShort);
        }

        let i = s[4..]
            .parse()
            .map_err(|_| ParseErr::InvalidNum)?;

        match &s[0..4] {
            "acc " => Ok(Instruction::Acc(i)),
            "jmp " => Ok(Instruction::Jmp(i)),
            "nop " => Ok(Instruction::Nop(i)),
            _ => Err(ParseErr::InvalidInstruction),
        }
    }
}

impl std::fmt::Display for ParseErr {
   fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
       match self {
           Self::TooShort => write!(fmt, "too short"),
           Self::InvalidNum => write!(fmt, "invalid num"),
           Self::InvalidInstruction => write!(fmt, "invalid instruction"),
       }
   }
}

impl std::error::Error for ParseErr {}
