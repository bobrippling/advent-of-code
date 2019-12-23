mod lib;
use lib::{IntCodeMachine, State, Word};

mod parse;

struct Nic {
    machine: IntCodeMachine,

    addr: Word,
    init: bool,

    input_queue: Vec<Packet>,
}

#[derive(Debug)]
struct Packet {
    addr: Word,
    x: Word,
    y: Word,
}

impl Packet {
    fn new(addr: Word, x: Word, y: Word) -> Self {
        Self { addr, x, y }
    }
}

impl Nic {
    fn new(addr: Word, mem: &[Word]) -> Self {
        Self {
            machine: IntCodeMachine::new(mem, false),

            addr,
            init: false,

            input_queue: vec![],
        }
    }

    fn run(&mut self) -> Vec<Packet> {
        let mut input = Vec::new();

        if !self.init {
            input.push(self.addr);
            self.init = true;
        }

        if self.input_queue.is_empty() {
            input.push(-1);
        } else {
            for packet in &self.input_queue {
                input.push(packet.x);
                input.push(packet.y);
            }
            self.input_queue.clear();
        }

        match self.machine.state() {
            State::Halted => {
                vec![]
            },
            State::Running => {
                let output = self.machine.interpret_async(&mut input);

                let mut packets = Vec::new();
                assert!(output.len() % 3 == 0);

                for i in (0..output.len()).step_by(3) {
                    let (dest, x, y) = (output[i], output[i+1], output[i+2]);

                    packets.push(Packet::new(dest, x, y));
                }

                packets
            },
        }
    }
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = parse::bytes("./input-day23")?;

    let min = 0;
    let max = 50;

    let mut nics = (min..max).map(
        |addr| Nic::new(addr, &bytes)
    ).collect::<Vec<_>>();

    'main: loop {
        for i in min..max {
            let packets = nics[i as usize].run();

            if !packets.is_empty() {
                println!("nic {}", i);
            }

            for packet in packets {
                println!("  packet {},{} --> {}", packet.x, packet.y, packet.addr);
                if min <= packet.addr && packet.addr < max {
                    nics[packet.addr as usize].input_queue.push(packet);
                } else if packet.addr == 255 {
                    println!("packet to addr 255: {:?}", packet);
                    break 'main;
                }
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1()?;

    Ok(())
}
