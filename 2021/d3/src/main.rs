type N = u16;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| {
            //str::parse
            let n = l.chars().rev().enumerate().fold(0 as N, |acc, (i, ch)| {
                let n = match ch {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!(),
                };
                acc | n << i
            });
            n
        })
        .collect::<Vec<_>>();

    // numbers
    //     .iter()
    //     .take(5)
    //     .for_each(|n| {
    //         println!("{:b}", n);
    //     });

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));

    Ok(())
}

fn part1(numbers: &Vec<N>) -> u32 {
    let mut out = [0usize; 12];

    for &n in numbers {
        for i in 0..12 {
            if n & (1 << i) != 0 {
                out[i] += 1;
            }
        }
    }

    // println!("{:?}, len = {}", out, numbers.len());

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..12 {
        if out[i] >= numbers.len() / 2 {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    // println!("gamma = {:b}", gamma);
    // println!("epsilon = {:b}", epsilon);

    gamma * epsilon
}

fn part2(numbers: &Vec<N>) -> u32 {
    const LIM: usize = 12;

    // println!("{:?}, len = {}", out, numbers.len());

    let mut oxygen = None;
    let mut co2 = None::<u32>;

    {
        let mut numbers: Vec<_> = numbers.iter().map(|n| (n, true)).collect();

        for i in (0..LIM).rev() {
            let mut out = [0usize; LIM];

            for (&n, _) in numbers.iter().filter(|(_, keep)| *keep) {
                for i in 0..LIM {
                    if n & (1 << i) != 0 {
                        out[i] += 1;
                    }
                }
            }
            let num_len = numbers.iter().filter(|(_, keep)| *keep).count();
            let most_common_bit = out[i] * 2 >= num_len;

            // let show = numbers.iter().copied().map(|(n, _)| format!("{:05b}", n)).collect::<Vec<_>>();
            // println!("numbers before: {:?}", show);
            let show = numbers.iter().copied().map(|(n, _)| format!("{:05b}", n)).collect::<Vec<_>>();
            // println!("out[{}] = {}, n = {}, keeping {}s\n{:?}", i, out[i], num_len, most_common_bit as u8, show);

            for (&n, keep) in numbers.iter_mut().filter(|(_, keep)| *keep) {
                let is_set = (n & (1 << i)) != 0;
                if is_set != most_common_bit {
                    // println!("*keep = f");
                    *keep = false;
                }
            }

            let keeps: Vec<_> = numbers.iter().filter(|(_, keep)| *keep).map(|(n, _)| n).copied().collect();
            let show = keeps.iter().copied().map(|n| format!("{:05b}", n)).collect::<Vec<_>>();
            // println!("keeps after bit {} (keeping {}s) = {:?}\n", i, most_common_bit as u8, show);
            if keeps.len() == 1 {
                oxygen = Some(*keeps[0] as u32);
                break;
            }
        }
    }

    {
        let mut numbers: Vec<_> = numbers.iter().map(|n| (n, true)).collect();

        for i in (0..LIM).rev() {
            let mut out = [0usize; LIM];

            for (&n, _) in numbers.iter().filter(|(_, keep)| *keep) {
                for i in 0..LIM {
                    if n & (1 << i) != 0 {
                        out[i] += 1;
                    }
                }
            }
            let num_len = numbers.iter().filter(|(_, keep)| *keep).count();
            let most_common_bit = out[i] * 2 < num_len;

            // let show = numbers.iter().copied().map(|(n, _)| format!("{:05b}", n)).collect::<Vec<_>>();
            // println!("numbers before: {:?}", show);
            let show = numbers.iter().copied().map(|(n, _)| format!("{:05b}", n)).collect::<Vec<_>>();
            // println!("out[{}] = {}, n = {}, keeping {}s\n{:?}", i, out[i], num_len, most_common_bit as u8, show);

            for (&n, keep) in numbers.iter_mut().filter(|(_, keep)| *keep) {
                let is_set = (n & (1 << i)) != 0;
                if is_set != most_common_bit {
                    // println!("*keep = f");
                    *keep = false;
                }
            }

            let keeps: Vec<_> = numbers.iter().filter(|(_, keep)| *keep).map(|(n, _)| n).copied().collect();
            let show = keeps.iter().copied().map(|n| format!("{:05b}", n)).collect::<Vec<_>>();
            // println!("keeps after bit {} (keeping {}s) = {:?}\n", i, most_common_bit as u8, show);
            if keeps.len() == 1 {
                co2 = Some(*keeps[0] as u32);
                break;
            }
        }
    }

    dbg!((oxygen, co2));
    oxygen.unwrap() * co2.unwrap()
}

#[cfg(test)]
mod test {
    static NUMBERS: [u16; 12] = [
        0b00100,
        0b11110,
        0b10110,
        0b10111,
        0b10101,
        0b01111,
        0b00111,
        0b11100,
        0b10000,
        0b11001,
        0b00010,
        0b01010,
    ];

    #[test]
    fn part2() {
        let v = NUMBERS.iter().copied().collect();
        let p2 = super::part2(&v);
        assert_eq!(p2, 230);
    }
}
