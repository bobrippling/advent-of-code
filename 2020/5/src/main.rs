use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BoardingPass {
    row: u16,
    col: u16,
}

impl BoardingPass {
    fn seat_id(&self) -> usize {
        self.row as usize * 8 + self.col as usize
    }
}

impl FromStr for BoardingPass {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<BoardingPass, Self::Err> {
        let mut row = 0;
        let mut col = 0;
        for (i, ch) in s.chars().enumerate() {
            match ch {
                'F' | 'B' => {
                    let bit = match ch {
                        'F' => 0,
                        'B' => 1,
                        _ => unreachable!(),
                    };
                    if i > 6 {
                        return Err(ParseErr::TooManyRows);
                    }
                    row |= bit << (6 - i);
                },
                'L' | 'R' => {
                    let bit = match ch {
                        'L' => 0,
                        'R' => 1,
                        _ => unreachable!(),
                    };
                    if i <= 6 || i > (6 + 3) {
                        return Err(ParseErr::TooManyCols);
                    }
                    col |= bit << (2 - (i - 7));
                },
                ch => return Err(ParseErr::InvalidChar(ch)),
            }
        }

        Ok(BoardingPass { row, col })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParseErr {
    InvalidChar(char),
    TooManyRows,
    TooManyCols,
}

impl std::error::Error for ParseErr {}

impl std::fmt::Display for ParseErr {
   fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
       match self {
           Self::InvalidChar(ch) => write!(fmt, "Invalid char '{}'", ch),
           Self::TooManyRows => write!(fmt, "Too many rows"),
           Self::TooManyCols => write!(fmt, "Too many rows"),
       }
   }
}

#[test]
fn test_parse() {
    let eg = "FBFBBFFRLR";

    assert_eq!(eg.parse(), Ok(BoardingPass { row: 44, col: 5 }));

    assert_eq!("BFFFBBFRRR".parse(), Ok(BoardingPass { row: 70, col: 7 }));
    assert_eq!(BoardingPass { row: 70, col: 7 }.seat_id(), 567);

    assert_eq!("FFFBBBFRRR".parse(), Ok(BoardingPass { row: 14, col: 7 }));
    assert_eq!(BoardingPass { row: 14, col: 7 }.seat_id(), 119);

    assert_eq!("BBFFBBFRLL".parse(), Ok(BoardingPass { row: 102, col: 4 }));
    assert_eq!(BoardingPass { row: 102, col: 4 }.seat_id(), 820);
}

fn part1<'a>(
    boarding_passes: impl Iterator<Item = &'a BoardingPass>
) {
    let largest_id = boarding_passes
        .map(BoardingPass::seat_id)
        .max()
        .unwrap();

    println!("Part 1: {}", largest_id);
}

fn part2<'a>(
    boarding_passes: impl Iterator<Item = &'a BoardingPass>
) {
    let mut sorted = boarding_passes
        .map(BoardingPass::seat_id)
        .collect::<Vec<_>>();
    sorted.sort();
    let sorted = sorted;

    let mut last = None;
    for ent in sorted {
        if let Some(last) = last {
            if last != 0 && last + 1 != ent {
                assert!(last + 1 == ent - 1);
                println!("Part 2: {}", last + 1);
                break;
            }
        }

        last = Some(ent);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;

    let boarding_passes = s
        .split("\n")
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    part1(boarding_passes.iter());
    part2(boarding_passes.iter());

    Ok(())
}
