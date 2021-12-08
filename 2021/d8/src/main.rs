use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Lines(Vec<Entry>);

#[derive(Debug)]
struct Entry {
    wire_combos: Vec<SevenSeg>,
    output: Vec<SevenSeg>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &Lines) -> usize {
    lines
        .0
        .iter()
        .flat_map(|Entry { output, .. }| output)
        .filter(|outputs| match outputs.on_count() {
            // '1': 2
            // '7': 3
            // '4': 4
            // '8': 7
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn part2(lines: &Lines) -> usize {
    for Entry { wire_combos, .. } in &lines.0 {
        let _mapping = solve_wiring(wire_combos);
    }

    todo!()
}

fn solve_wiring(wire_combos: &Vec<SevenSeg>) -> HashMap<char, char> {
    let mut wire_possibilities = HashMap::<char, HashSet<char>>::new();

    let mut seen_targets = HashSet::new();

    let mut eliminate_digit = |canonical_seg: &SevenSeg| {
        let matching = wire_combos
            .iter()
            .filter(|input| input.on_count() == canonical_seg.on_count())
            .collect::<Vec<_>>();

        if let [target] = matching[..] {
            println!(
                "'{:?}' (target) matches '{:?}' (canon)",
                target, canonical_seg
            );

            let canon_chars = canonical_seg.chars().collect();

            for target_ch in target.chars() {
                match wire_possibilities.get_mut(&target_ch) {
                    Some(set) => {
                        // it already exists, narrow down possibilities
                        let narrowed = set.intersection(&canon_chars).copied().collect();
                        *set = narrowed;

                        println!(
                            "  {} => {:?}    (narrowing down previous matches)",
                            target_ch, *set
                        );

                        // *set = set.iter().copied().filter(|x| canon_chars.contains(x)).collect();
                    }
                    None => {
                        // doesn't exist, create initial possibilities.
                        let mut possibilities = canon_chars.clone();

                        if !seen_targets.is_empty() {
                            // we also exclude target chars we've already seen, since we can't map to them anyway
                            possibilities =
                                possibilities.difference(&seen_targets).copied().collect();
                            println!(
                                "  {} => {:?}    (already seen targets {:?})",
                                target_ch, possibilities, seen_targets
                            );
                        } else {
                            println!(
                                "  {} => {:?}    (no targets seen so far)",
                                target_ch, possibilities
                            );
                        }

                        wire_possibilities.insert(target_ch, possibilities);
                    }
                };
            }

            for &ch in &canon_chars {
                seen_targets.insert(ch);
            }
        } else {
            panic!("expected a single match")
        }
    };

    eliminate_digit(&SevenSeg::_1);
    eliminate_digit(&SevenSeg::_4);
    eliminate_digit(&SevenSeg::_7);
    eliminate_digit(&SevenSeg::_8);

    let remaining_lengths = [
        &SevenSeg::_0,
        // &SevenSeg::_1,
        &SevenSeg::_2,
        &SevenSeg::_3,
        // &SevenSeg::_4,
        &SevenSeg::_5,
        &SevenSeg::_6,
        // &SevenSeg::_7,
        // &SevenSeg::_8,
        &SevenSeg::_9,
    ]
    .iter()
    .copied()
    .map(SevenSeg::on_count)
    .collect::<HashSet<_>>();

    loop {
        let done = wire_possibilities.values().all(|set| set.len() == 1);
        if done {
            break;
        }

        let (&ch, target) = wire_possibilities
            .iter()
            .find(|(_, set)| set.len() == 1)
            .expect("couldn't narrow down");

        // find all the segments where both of these are on
        let target_ch = *target.iter().next().unwrap();

        let candidates = SevenSeg::all()
            .filter(|seg| seg.is_on(ch) && seg.is_on(target_ch))
            .collect::<Vec<_>>();

        println!("{} is {}", ch, target_ch);
        println!("giving {} candidates", candidates.len());

        todo!()
    }

    wire_possibilities
        .iter()
        .map(|(k, set)| (*k, *set.iter().next().unwrap()))
        .collect()
}

#[derive(Default)]
struct SevenSeg([bool; 7]);

impl SevenSeg {
    // count the number of on-bits
    fn on_count(&self) -> u8 {
        self.on_bits().count() as _
    }

    fn is_on(&self, ch: char) -> bool {
        self.0[Self::char_to_index(ch).unwrap() as usize]
    }

    // // returns a bitmask,
    // // e.g. if we have two on bits:
    // //   '1' requires two on bits, so 1 << 1
    // // e.g. if we have five on bits:
    // //   '2', '3' and '5' require five bits, so (1 << 2 | 1 << 3 | 1 << 5)
    // fn on_count_mask(&self) -> u8 {
    //     self.on_bits()
    //         .map(|i| 1 << i)
    //         .reduce(std::ops::BitOr::bitor)
    //         .unwrap()
    // }

    fn on_bits(&self) -> impl Iterator<Item = u8> + '_ {
        self.0
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(i, _)| i as u8)
    }
}

/*
 *   aa
 *  b  c
 *   dd
 *  e  f
 *   gg
 *
 * 0: a b c   e f g (6)
 * 1:     c     f   (2)
 * 2: a   c d e g   (5)
 * 3: a   c d   f g (5)
 * 4: b   c d   f   (4)
 * 5: a b   d   f g (5)
 * 6: a b   d e f g (6)
 * 7: a   c     f   (3)
 * 8: a b c d e f g (7)
 * 9: a b c d   f g (6)
 */
impl SevenSeg {
    const _0: Self = Self([true, true, true, false, true, true, true]); // a b c e f g
    const _1: Self = Self([false, false, true, false, false, true, false]); // c f
    const _2: Self = Self([true, false, true, true, true, false, true]); // a c d e g
    const _3: Self = Self([true, false, true, true, false, true, true]); // a c d f g
    const _4: Self = Self([false, true, true, true, false, true, false]); // b c d f
    const _5: Self = Self([true, true, false, true, false, true, true]); // a b d f g
    const _6: Self = Self([true, true, false, true, true, true, true]); // a b d e f g
    const _7: Self = Self([true, false, true, false, false, true, false]); // a c f
    const _8: Self = Self([true, true, true, true, true, true, true]); // a b c d e f g
    const _9: Self = Self([true, true, true, true, false, true, true]); // a b c d f g

    fn all() -> impl Iterator<Item = &'static SevenSeg> {
        [
            Self::_0,
            Self::_1,
            Self::_2,
            Self::_3,
            Self::_4,
            Self::_5,
            Self::_6,
            Self::_7,
            Self::_8,
            Self::_9,
        ]
        .iter()
    }
}

impl std::fmt::Debug for SevenSeg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &v) in self.0.iter().enumerate() {
            if v {
                let ch = b'a' + i as u8;
                write!(f, "{}", ch as char)?;
            }
        }
        Ok(())
    }
}

impl std::str::FromStr for Lines {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_sevensegs = |s: &str| s.split(' ').map(str::parse).collect::<Result<Vec<_>, _>>();

        Ok(Self(
            s.lines()
                .map(|l| {
                    let parts: Vec<_> = l.trim().split(" | ").collect();

                    if let [input, output] = parts[..] {
                        Ok(Entry {
                            wire_combos: parse_sevensegs(input)?,
                            output: parse_sevensegs(output)?,
                        })
                    } else {
                        Err("wrong split count for line")
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl SevenSeg {
    fn char_to_index(ch: char) -> Option<usize> {
        Some(match ch {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => return None,
        })
    }

    fn index_to_char(i: usize) -> Option<char> {
        Some(match i {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            _ => return None,
        })
    }

    fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.on_bits()
            .map(|b| b as usize)
            .map(Self::index_to_char)
            .map(Option::unwrap)
    }
}

impl std::str::FromStr for SevenSeg {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sevenseg = SevenSeg::default();

        for ch in s.chars() {
            let i = SevenSeg::char_to_index(ch).ok_or("invalid char")?;

            sevenseg.0[i] = true;
        }

        Ok(sevenseg)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG: &'static str = "\
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\
    ";

    #[test]
    fn test_part1() {
        let lines = EG.parse().unwrap();
        println!("Parsed: {:?}", lines);
        assert_eq!(part1(&lines), 26);
    }

    #[test]
    fn test_part2() {
        let lines = EG.parse().unwrap();
        assert_eq!(part2(&lines), 61229);
    }
}
