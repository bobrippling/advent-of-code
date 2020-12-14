use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

type Error = Box<dyn std::error::Error>;

type T = u64; // 36-bit unsigned
type Addr = u64; // 36-bit unsigned

#[derive(Debug)]
enum Instruction {
    SetMask {
        and: T,
        or: T,
    },
    SetMem {
        addr: Addr,
        val: T,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mask {
    One,
    Zero,
    Floating,
}

#[derive(Debug)]
enum Instruction2 {
    SetMask(Vec<Mask>),
    SetMem {
        addr: Addr,
        val: T,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;

    println!("Part 1: {}", part1(&(parse(&s)?)));
    println!("Part 2: {}", part2(&(parse2(&s)?)));

    Ok(())
}

fn parse(s: &str) -> Result<Vec<Instruction>, Error> {
    s
        .lines()
        .map(|line| {
            if line.starts_with("mask = ") {
                let mask = &line[7..];
                let mut or = 0;
                let mut and = !0;

                for (i, ch) in mask.chars().rev().enumerate() {
                    match ch {
                        'X' => {}
                        '0' => and &= !(1 << i),
                        '1' => or  |=   1 << i,
                        _ => return Err("invalid char".into()),
                    }
                }
                Ok(Instruction::SetMask { and, or })
            } else if line.starts_with("mem[") {
                let parts = line[4..].split("] = ").collect::<Vec<_>>();

                if parts.len() != 2 {
                    return Err("invalid mem line".into());
                }

                let addr = parts[0].parse()?;
                let val = parts[1].parse()?;

                Ok(Instruction::SetMem { addr, val })
            } else {
                Err("invalid line".into())
            }
        })
        .collect::<Result<Vec<_>, Box<_>>>()
}

fn part1(instructions: &[Instruction]) -> T {
    let mut mem = HashMap::<Addr, T>::new();
    let mut mask_and = !0;
    let mut mask_or = 0;

    for i in instructions {
        match *i {
            Instruction::SetMask { and, or, } => {
                mask_and = and;
                mask_or = or;
            },
            Instruction::SetMem { addr, val, } => {
                let valp = mem.entry(addr).or_insert(0);

                *valp = (val & mask_and) | mask_or;
            },
        }
    }

    mem.values().sum()
}

#[test]
fn test_part1() {
    let eg = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    assert_eq!(part1(&parse(&eg).unwrap()), 165);
}

fn parse2(s: &str) -> Result<Vec<Instruction2>, Error> {
    Ok(s
        .lines()
        .map(|line| {
            if line.starts_with("mask = ") {
                let mask = &line[7..];

                let mask = mask.chars()
                    .map(|ch| {
                        match ch {
                            'X' => Ok(Mask::Floating),
                            '0' => Ok(Mask::Zero),
                            '1' => Ok(Mask::One),
                            _ => return Err("invalid char"),
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Instruction2::SetMask(mask))
            } else if line.starts_with("mem[") {
                let parts = line[4..].split("] = ").collect::<Vec<_>>();

                if parts.len() != 2 {
                    return Err("invalid mem line");
                }

                let addr = parts[0].parse().unwrap();
                let val = parts[1].parse().unwrap();

                Ok(Instruction2::SetMem { addr, val })
            } else {
                Err("invalid line")
            }
        })
        .collect::<Result<Vec<_>, _>>()?)
}
fn part2(instructions: &[Instruction2]) -> T {
    let mut mem = HashMap::<Addr, T>::new();
    let mut mask = Vec::new();

    for i in instructions {
        match i {
            Instruction2::SetMask(newmask) => {
                mask = newmask.clone();
            },
            Instruction2::SetMem { addr, val, } => {
                for addr in addr_combinations(*addr, &mask) {
                    let valp = mem.entry(addr).or_insert(0);
                    *valp = *val;
                }
            },
        }
    }

    mem.values().sum()
}

fn addr_combinations<'a>(addr: Addr, mask: &'a Vec<Mask>) -> impl Iterator<Item = Addr> + 'a {
    let num_floatings = mask.iter().filter(|&&m| m == Mask::Floating).count();
    let combos = 2_usize.pow(num_floatings as u32);

    struct Comb<'a> {
        i: usize,
        max: usize,
        mask: &'a Vec<Mask>,
        base: Addr,
    }

    impl Comb<'_> {
        fn addr_from_index(&self, index: usize) -> Addr {
            let mut addr = self.base;
            let mut float_i = 0;

            for (i, mask_ch) in self.mask.iter().rev().enumerate() {
                match mask_ch {
                    Mask::Zero => {},
                    Mask::One => {
                        addr |= 1 << i;
                    },
                    Mask::Floating => {
                        if index & (1 << float_i) == 0 {
                            addr &= !(1 << i);
                        } else {
                            addr |= 1 << i;
                        }
                        float_i += 1;
                    }
                }
            }

            addr
        }
    }

    impl<'a> Iterator for Comb<'a> {
        type Item = Addr;

        fn next(&mut self) -> Option<Self::Item> {
            if self.i == self.max {
                return None;
            }

            self.i += 1;
            Some(self.addr_from_index(self.i - 1))
        }
    }

    Comb {
        i: 0,
        max: combos,
        mask,
        base: addr,
    }
}

#[test]
fn test_addr_combinations() {
    let masks = "000000000000000000000000000000X1001X"
        .chars()
        .map(|ch| match ch {
            '0' => Mask::Zero,
            '1' => Mask::One,
            'X' => Mask::Floating,
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    let combs = addr_combinations(0b0101010, &masks);

    let mut expected = HashSet::new();
    expected.insert(0b00011010);
    expected.insert(0b00011011);
    expected.insert(0b00111010);
    expected.insert(0b00111011);

    assert_eq!(
        combs.collect::<HashSet<_>>(),
        expected,
    );
}

#[test]
fn test_part2() {
    let eg = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    assert_eq!(part2(&parse2(&eg).unwrap()), 208);
}
