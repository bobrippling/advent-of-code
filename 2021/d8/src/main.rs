#[derive(Debug)]
struct Lines {
    entries: Vec<(Vec<Digits>, Vec<Digits>)>,
    //        signal patterns, output value
}

#[derive(Debug)]
struct Digits(Vec<Digit>);

#[derive(Debug)]
enum Digit {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

struct SevenSeg([bool; 7]);

static DIGITS: [SevenSeg; 10] = [
    SevenSeg([true, true, true, false, true, true, true]), // a b c e f g
    SevenSeg([false, false, true, false, false, true, false]), // c f
    SevenSeg([true, false, true, true, true, false, true]), // a c d e g
    SevenSeg([true, false, true, true, false, true, true]), // a c d f g
    SevenSeg([false, true, true, true, false, true, false]), // b c d f
    SevenSeg([true, true, false, true, false, true, true]), // a b d f g
    SevenSeg([true, true, false, true, true, true, true]), // a b d e f g
    SevenSeg([true, false, true, false, false, true, false]), // a c f
    SevenSeg([true, true, true, true, true, true, true]),  // a b c d e f g
    SevenSeg([true, true, true, true, false, true, true]), // a b c d f g
];

static DIGIT_COUNTS: [u8; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));

    Ok(())
}

fn part1(lines: &Lines) -> usize {
    lines
        .entries
        .iter()
        .flat_map(|(_, output_signals)| output_signals)
        .filter(|outputs| match outputs.0.len() {
            // 2: '1'
            // 3: '7'
            // 4: '4'
            // 7: '8'
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn part2(lines: &Lines) -> usize {
    /*
     *   aa
     *  b  c
     *   dd
     *  e  f
     *   gg
     *
     * 0: a b c e f g (6)
     * 1: c f (2)
     * 2: a c d e g (5)
     * 3: a c d f g (5)
     * 4: b c d f (4)
     * 5: a b d f g (5)
     * 6: a b d e f g (6)
     * 7: a c f (3)
     * 8: a b c d e f g (7)
     * 9: a b c d f g (6)
     */

    for (inputs, outputs) in &lines.entries {
        let mut found_digits = [None; 10];

        let mut find_digit = |digit, count| {
            let matching = inputs
                .iter()
                .filter(|input| input.0.len() == count)
                .collect::<Vec<_>>();

            assert!(matching.len() == 1);

            found_digits[digit] = Some(matching[0]);
        };

        find_digit(1, 2); // '1'
        find_digit(7, 3); // '7'
        find_digit(4, 4); // '4'
        find_digit(8, 7); // '8'

        println!("got these: {:?}", found_digits);
        loop {

        }

        break;
    }

    todo!()
}

impl std::str::FromStr for Lines {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            entries: s
                .lines()
                .map(|l| {
                    let parts: Vec<_> = l.trim().split(" | ").collect();

                    if let [input, output] = parts[..] {
                        (
                            input
                                .split(' ')
                                .map(str::parse)
                                .map(Result::unwrap)
                                .collect(),
                            output
                                .split(' ')
                                .map(str::parse)
                                .map(Result::unwrap)
                                .collect(),
                        )
                    } else {
                        panic!()
                    }
                })
                .collect(),
        })
    }
}

impl std::str::FromStr for Digits {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|c| match c {
                    'a' => Digit::A,
                    'b' => Digit::B,
                    'c' => Digit::C,
                    'd' => Digit::D,
                    'e' => Digit::E,
                    'f' => Digit::F,
                    'g' => Digit::G,
                    _ => panic!(),
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //static EG: &'static str = "\
    //    acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\
    //";
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
