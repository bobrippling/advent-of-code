mod lib;
use lib::{IntCodeMachine, State, Word};

mod parse;

#[cfg(test)]
use lib::interpret_oneshot;

#[cfg(test)]
type Phase = Vec<Word>;

#[cfg(test)]
fn run_phase(phases: &Vec<Word>, bytes: &[Word]) -> Word {
    let mut last_i = 0;

    for phase in 0..=4 {
        let mut bytes = Vec::from(bytes);
        let mut input = vec![phases[phase], last_i];

        let output = interpret_oneshot(&mut bytes, &mut input);
        assert_eq!(input.len(), 0);
        assert_eq!(output.len(), 1);
        let old = last_i;
        last_i = output[0];
        println!("  phase {:?}, input {} output {}",
                 phases[phase], old, last_i);
    }

    last_i
}

#[cfg(test)]
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
    struct Amplifier {
        machine: IntCodeMachine,

        input_queue: Vec<Word>,
        output_queue: Vec<Word>,
    };

    impl Amplifier {
        fn new(phase: Word, mem: &[Word]) -> Self {
            Self {
                machine: IntCodeMachine::new(mem, false),

                input_queue: vec![phase],
                output_queue: vec![],
            }
        }

        fn run(&mut self) {
            match self.machine.state() {
                State::Running => {
                    let mut output = self.machine.interpret_async(&mut self.input_queue);

                    self.output_queue.append(&mut output);
                },
                State::Halted => {
                },
            };
        }

        fn running(&self) -> bool {
            match self.machine.state() {
                State::Running => true,
                State::Halted => false,
            }
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
            if amp.running() {
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
    let bytes = parse::bytes("./input-day7")?;

    let (max, phase) = find_max_phase_feedback(&bytes);
    println!("max={} phasse={:?}", max, phase);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_phase() {
        let mut bytes = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];

        let output = interpret_oneshot(&mut bytes, &mut vec![4, 0]);

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 4);
    }

    #[test]
    fn test_day7_part1_eg0() {
        let expected_max = 43210;
        let expected_phase = [4,3,2,1,0];
        let bytes = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];

        let (max, phase) = find_max_phase(&bytes);
        assert_eq!(phase, expected_phase);
        assert_eq!(max, expected_max);
    }

    #[test]
    fn test_day7_part1_eg1() {
        let expected_max = 54321;
        let expected_phase = [0,1,2,3,4];
        let mut bytes = [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];

        let (max, phase) = find_max_phase(&mut bytes);
        assert_eq!(max, expected_max);
        assert_eq!(phase, expected_phase);
    }

    #[test]
    fn test_day7_part1_eg2() {
        let expected_max = 65210;
        let expected_phase = [1,0,4,3,2];
        let mut bytes = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];

        let (max, phase) = find_max_phase(&mut bytes);
        assert_eq!(max, expected_max);
        assert_eq!(phase, expected_phase);
    }

    // -----

    #[test]
    fn test_day7_part2_eg1() {
        let expected_max = 139629729;
        let expected_phase = [9,8,7,6,5];
        let bytes = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26, 27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];

        let (max, phase) = find_max_phase_feedback(&bytes);

        assert_eq!(max, expected_max);
        assert_eq!(phase, expected_phase);
    }

    #[test]
    fn test_day7_part2_eg2() {
        let expected_max = 18216;
        let expected_phase = [9,7,8,5,6];
        let bytes = [ 3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10 ];

        let (max, phase) = find_max_phase_feedback(&bytes);

        assert_eq!(max, expected_max);
        assert_eq!(phase, expected_phase);
    }
}
