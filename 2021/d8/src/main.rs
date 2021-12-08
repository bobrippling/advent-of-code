use std::collections::HashSet;

struct Lines(Vec<Entry>);

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
        .filter(|outputs| match outputs.segments_on() {
            // '1': 2
            // '4': 4
            // '7': 3
            // '8': 7
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn part2(lines: &Lines) -> usize {
    lines
        .0
        .iter()
        .map(|entry| {
            let Entry {
                wire_combos,
                output,
            } = entry;

            let solution = solve_wiring(wire_combos);

            output
                .iter()
                .map(|seg| {
                    solution
                        .iter()
                        .copied()
                        .enumerate()
                        .find(|&(_, encoded)| encoded == seg)
                        .map(|(i, _)| i)
                        .unwrap()
                })
                .fold(0, |acc, n| acc * 10 + n)
        })
        .sum()
}

fn solve_wiring(wire_combos: &Vec<SevenSeg>) -> Vec<&SevenSeg> {
    let find_with_count = |count| {
        wire_combos
            .iter()
            .find(|&seg| seg.segments_on() == count)
            .unwrap()
    };

    let one = find_with_count(SevenSeg::_1.segments_on());
    let four = find_with_count(SevenSeg::_4.segments_on());
    let seven = find_with_count(SevenSeg::_7.segments_on());
    let eight = find_with_count(SevenSeg::_8.segments_on());

    fn find_matching<'a>(from: &HashSet<&'a SevenSeg>, seg: &SevenSeg) -> &'a SevenSeg {
        let matches = from
            .iter()
            .copied()
            .filter(|&candidate| (candidate & seg) == *seg)
            .collect::<HashSet<_>>();

        assert_eq!(matches.len(), 1);

        matches.iter().next().unwrap()
    }

    let nine = find_matching(
        &wire_combos
            .iter()
            .filter(|&seg| seg != eight && seg != four)
            .collect(),
        &four,
    );

    let bd = four - one; // two leftmost segments of '4'

    // segments with five on
    let (two, three, five);
    {
        // 2, 3 and 5
        let five_seg = wire_combos
            .iter()
            .filter(|seg| seg.segments_on() == 5)
            .collect::<HashSet<_>>();

        assert_eq!(five_seg.len(), 3);

        three = find_matching(&five_seg, &one);
        five = find_matching(&five_seg, &bd);

        two = five_seg
            .iter()
            .copied()
            .find(|&seg| seg != three && seg != five)
            .unwrap();
    }

    // segments with six on
    let (six, zero); // already got nine
    {
        let six_seg = wire_combos
            .iter()
            .filter(|seg| seg.segments_on() == 6)
            .filter(|seg| seg != &nine)
            .collect::<HashSet<_>>();

        six = find_matching(&six_seg, &bd);
        zero = six_seg
            .iter()
            .copied()
            .find(|&seg| seg != six)
            .unwrap();
    }

    vec![zero, one, two, three, four, five, six, seven, eight, nine]
}

#[derive(Default, Hash, Eq, PartialEq, Clone)]
struct SevenSeg([bool; 7]);

impl SevenSeg {
    // count the number of on-bits
    fn segments_on(&self) -> u8 {
        self.bits_on().count() as _
    }

    fn bits_on(&self) -> impl Iterator<Item = u8> + '_ {
        self.0
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(i, _)| i as u8)
    }
}

impl std::ops::BitAnd for &SevenSeg {
    type Output = SevenSeg;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        for (a, &b) in out.0.iter_mut().zip(rhs.0.iter()) {
            *a &= b;
        }
        out
    }
}

impl std::ops::Sub for &SevenSeg {
    type Output = SevenSeg;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        for (a, &b) in out.0.iter_mut().zip(rhs.0.iter()) {
            if b {
                *a = false;
            }
        }
        out
    }
}

impl std::ops::Not for &SevenSeg {
    type Output = SevenSeg;

    fn not(self) -> Self::Output {
        let mut out = self.clone();
        for x in &mut out.0 {
            *x = !*x;
        }
        out
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
 * 2: a   c d e   g (5)
 * 3: a   c d   f g (5)
 * 4:   b c d   f   (4)
 * 5: a b   d   f g (5)
 * 6: a b   d e f g (6)
 * 7: a   c     f   (3)
 * 8: a b c d e f g (7)
 * 9: a b c d   f g (6)
 */
impl SevenSeg {
    #![allow(unused_variables)]

    const _0: Self = Self([true, true, true, false, true, true, true]);
    const _1: Self = Self([false, false, true, false, false, true, false]);
    const _2: Self = Self([true, false, true, true, true, false, true]);
    const _3: Self = Self([true, false, true, true, false, true, true]);
    const _4: Self = Self([false, true, true, true, false, true, false]);
    const _5: Self = Self([true, true, false, true, false, true, true]);
    const _6: Self = Self([true, true, false, true, true, true, true]);
    const _7: Self = Self([true, false, true, false, false, true, false]);
    const _8: Self = Self([true, true, true, true, true, true, true]);
    const _9: Self = Self([true, true, true, true, false, true, true]);
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
        assert_eq!(part1(&lines), 26);
    }

    #[test]
    fn test_part2() {
        let lines = EG.parse().unwrap();
        assert_eq!(part2(&lines), 61229);
    }
}
