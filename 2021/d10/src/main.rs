type Input = Vec<Result<Parsed, ParseError>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?
        .lines()
        .map(str::parse)
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(Result::as_ref)
        .filter_map(Result::err)
        .map(Score::score)
        .sum()
}

fn part2(input: &Input) -> u64 {
    let mut scores: Vec<_> = input
        .iter()
        .map(Result::as_ref)
        .filter_map(Result::ok)
        .map(|Parsed(stack)| {
            stack
                .iter()
                .rev()
                .map(Score::score)
                .map(|s| s as u64)
                .fold(0, |acc, s| acc * 5 + s)
        })
        .collect();

    scores.sort();

    let mid = scores.len() / 2;
    scores[mid]
}

trait Score {
    fn score(&self) -> u32;
}

struct Parsed(Vec<Bracket>);

#[derive(PartialEq, Eq, Debug)]
enum Bracket {
    Paren,
    Square,
    Curly,
    Angle,
}

impl TryFrom<char> for Bracket {
    type Error = char;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        use Bracket::*;
        Ok(match ch {
            '(' => Paren,
            '[' => Square,
            '{' => Curly,
            '<' => Angle,
            _ => return Err(ch),
        })
    }
}

impl Score for Bracket {
    fn score(&self) -> u32 {
        use Bracket::*;

        match self {
            Paren => 1,
            Square => 2,
            Curly => 3,
            Angle => 4,
        }
    }
}

enum ParseError {
    Expected(Bracket),
    EarlyClose(Bracket),
    UnexpectedChar(char),
}

impl Score for ParseError {
    fn score(&self) -> u32 {
        use Bracket::*;
        use ParseError::*;

        match self {
            Expected(ch) => match ch {
                Paren => 3,
                Square => 57,
                Curly => 1197,
                Angle => 25137,
            },
            EarlyClose(bracket) => panic!("don't know how to score EarlyClose({:?})", bracket),
            UnexpectedChar(ch) => panic!("don't know how to score UnexpectedChar({})", ch),
        }
    }
}

impl std::str::FromStr for Parsed {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![];

        let pop = |stack: &mut Vec<_>, bracket| {
            if let Some(popped) = stack.pop() {
                if popped == bracket {
                    Ok(())
                } else {
                    Err(ParseError::Expected(bracket))
                }
            } else {
                Err(ParseError::EarlyClose(bracket))
            }
        };

        for ch in s.chars() {
            if let Ok(open_bracket) = ch.try_into() {
                stack.push(open_bracket);
            } else {
                match ch {
                    ')' => pop(&mut stack, Bracket::Paren)?,
                    ']' => pop(&mut stack, Bracket::Square)?,
                    '}' => pop(&mut stack, Bracket::Curly)?,
                    '>' => pop(&mut stack, Bracket::Angle)?,
                    _ => return Err(ParseError::UnexpectedChar(ch)),
                };
            }
        }

        Ok(Parsed(stack))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG: &'static str = "\
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]\
    ";

    #[test]
    fn test_part1() {
        let input = EG
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Vec<_>>();
        assert_eq!(part1(&input), 26397);
    }

    #[test]
    fn test_part2() {
        let input = EG
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Vec<_>>();
        assert_eq!(part2(&input), 288957);
    }
}
