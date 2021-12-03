type N = u16;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let numbers = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|l| N::from_str_radix(l, 2))
        .collect::<Result<Vec<_>, _>>()?;

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));

    Ok(())
}

fn part1(numbers: &[N]) -> u32 {
    let highest_bit = highest_set_bit(numbers.iter().copied());
    let highest_bit = highest_bit + 1; // index to count
    let bit_counts = bit_counts(numbers.iter().copied(), highest_bit);

    #[derive(Default)]
    struct Diag {
        gamma: u32,
        epsilon: u32,
    }

    let diag = bit_counts
        .iter()
        .copied()
        .enumerate()
        .fold(Diag::default(), |diag, (i, n_ones)| {
            let most_common_is_1 = n_ones as usize >= numbers.len() / 2;

            if most_common_is_1 {
                Diag {
                    gamma: diag.gamma | 1 << i,
                    ..diag
                }
            } else {
                Diag {
                    epsilon: diag.epsilon | 1 << i,
                    ..diag
                }
            }
        });

    diag.gamma * diag.epsilon
}

fn part2(numbers: &[N]) -> u32 {
    let highest_bit = highest_set_bit(numbers.iter().copied());
    let highest_bit = highest_bit + 1; // index to count

    let oxygen = find_rating(
        numbers,
        highest_bit,
        |entries,
         &BitEntry {
             n_ones,
             i,
             kept_count,
         }| {
            let most_common_bit = n_ones * 2 >= kept_count;

            for (n, keep) in entries {
                if !*keep {
                    continue;
                }

                let current_bit = (*n & (1 << i)) != 0;
                if current_bit != most_common_bit {
                    *keep = false;
                }
            }
        },
    );

    let co2 = find_rating(
        numbers,
        highest_bit,
        |entries,
         &BitEntry {
             n_ones,
             i,
             kept_count,
         }| {
            let least_common_bit = n_ones * 2 < kept_count;

            for (n, keep) in entries {
                if !*keep {
                    continue;
                }

                let current_bit = (*n & (1 << i)) != 0;
                if current_bit != least_common_bit {
                    *keep = false;
                }
            }
        },
    );

    oxygen * co2
}

struct BitEntry {
    n_ones: usize,
    i: u32,
    kept_count: usize,
}

fn find_rating(
    numbers: &[N],
    highest_bit: u32,
    filter_down: impl Fn(&mut [(N, bool)], &BitEntry),
) -> u32 {
    let mut numbers: Vec<_> = numbers.iter().copied().map(|n| (n, true)).collect();

    for i in (0..highest_bit).rev() {
        let kept_entries: Vec<_> = numbers.iter().copied().kept_entries().collect();
        let bit_counts = bit_counts(kept_entries.iter().copied(), highest_bit);

        filter_down(
            &mut numbers,
            &BitEntry {
                n_ones: bit_counts[i as usize],
                i,
                kept_count: kept_entries.len(),
            },
        );

        let remaining: Vec<_> = numbers.iter().copied().kept_entries().collect();
        if let [single] = remaining[..] {
            return single as _;
        }
    }

    panic!("couldn't narrow down enough")
}

fn highest_set_bit(numbers: impl Iterator<Item = N>) -> u32 {
    numbers.fold(0, |acc, n| {
        if n != 0 {
            acc.max(n.highest_set_bit())
        } else {
            acc
        }
    })
}

trait HighestSetBit {
    fn highest_set_bit(self) -> u32;
}

impl HighestSetBit for N {
    fn highest_set_bit(self) -> u32 {
        Self::BITS - self.leading_zeros() - 1
    }
}

fn bit_counts<I>(numbers: I, highest_bit: u32) -> Vec<usize>
where
    I: Iterator<Item = N>,
{
    let mut bit_counts = vec![0usize; highest_bit as usize];

    for n in numbers {
        for i in 0..highest_bit {
            if n & (1 << i) != 0 {
                bit_counts[i as usize] += 1;
            }
        }
    }

    bit_counts
}

trait Keepable<I> {
    fn kept_entries(self) -> KeptEntries<I>;
}

struct KeptEntries<I> {
    iter: I,
}

impl<I> Keepable<I> for I {
    fn kept_entries(self) -> KeptEntries<I> {
        KeptEntries { iter: self }
    }
}

impl<I, T> Iterator for KeptEntries<I>
where
    I: Iterator<Item = (T, bool)>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some((x, true)) => break Some(x),
                Some((..)) => {}
                None => break None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let numbers = [
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];

        let p2 = super::part2(&numbers);
        assert_eq!(p2, 230);
    }

    #[test]
    fn highest_set_bit() {
        assert_eq!(super::highest_set_bit([0b100].iter().copied()), 2);
        assert_eq!(super::highest_set_bit([0].iter().copied()), 0);
        assert_eq!(super::highest_set_bit([0b10000].iter().copied()), 4);
    }

    #[test]
    fn test_kept_entries() {
        let ents = [(1, true), (2, true), (3, false), (4, true)];

        let kept: Vec<_> = ents.iter().copied().kept_entries().collect();

        assert_eq!(kept, vec![1, 2, 4]);
    }
}
