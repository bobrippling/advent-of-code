fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    println!("Part 1: {}", part1(&lines));
    // println!("Part 2: {}", part2(&lines));

    Ok(())
}

#[derive(Eq, PartialEq, Clone)]
enum Number {
    N(i64),
    Box(Box<Number>, Box<Number>),
}

fn part1(input: &[Number]) -> i64 {
    let total: Number = input.into_iter().cloned().sum();

    total.magnitude()
}

impl std::ops::Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut r = Self::Box(Box::new(self), Box::new(rhs));
        r.reduce();
        r
    }
}

impl std::iter::Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, n| {
            acc + n
        }).unwrap_or_default()
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::N(0)
    }
}

impl Number {
    fn reduce(&mut self) {
        // nest depth of 4 --> explode
        // n >= 10 --> split

        loop {
            while self.explode(0).is_some() {}

            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self, depth: u32) -> Option<(Option<i64>, Option<i64>)> {
        assert!(depth < 5);
        if depth == 4 {
            // [[[[ [9,8],1],2],3],4]
            // [[[[   0  ,9],2],3],4]

            match self.take_box() {
                Some((a, b)) => Some((Some(a), Some(b))),
                None => None,
            }
        } else {
            match self {
                Self::N(_) => None,
                Self::Box(a, b) => {
                    match a.explode(depth + 1) {
                        Some((apply_a, Some(apply_b))) => {
                            let apply_b = if b.add_left(apply_b) {
                                None
                            } else {
                                Some(apply_b)
                            };
                            return Some((apply_a, apply_b));
                        }
                        r @ Some(_) => return r,
                        None => {}
                    }
                    match b.explode(depth + 1) {
                        Some((Some(apply_a), apply_b)) => {
                            let apply_a = if a.add_right(apply_a) {
                                None
                            } else {
                                Some(apply_a)
                            };
                            return Some((apply_a, apply_b));
                        }
                        r @ Some(_) => return r,
                        None => {}
                    }
                    None
                }
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Self::N(n) => *n,
            Self::Box(a, b) => {
                3 * a.magnitude() + 2 * b.magnitude()
            }
        }
    }

    fn take_box(&mut self) -> Option<(i64, i64)> {
        match std::mem::replace(self, Self::N(0)) {
            Self::N(n) => {
                *self = Self::N(n);
                None
            }
            Self::Box(a, b) => Some(match (*a, *b) {
                (Self::N(a), Self::N(b)) => (a, b),
                _ => panic!("4-deep, not pair[n, n]"),
            }),
        }
    }

    fn add_left(&mut self, n: i64) -> bool {
        match self {
            Self::N(x) => {
                *x += n;
                true
            }
            Self::Box(a, _) => a.add_left(n),
        }
    }

    fn add_right(&mut self, n: i64) -> bool {
        match self {
            Self::N(x) => {
                *x += n;
                true
            }
            Self::Box(_, b) => b.add_right(n),
        }
    }

    fn split(&mut self) -> bool {
        match *self {
            Self::N(n) if n >= 10 => {
                let a = n / 2;
                let b = (n + 1) / 2;

                *self = Self::Box(Box::new(Number::N(a)), Box::new(Number::N(b)));
                true
            }
            Self::N(_) => false,
            Self::Box(ref mut a, ref mut b) => a.split() || b.split(),
        }
    }
}

impl std::str::FromStr for Number {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.bytes().collect::<Vec<_>>();

        let (n, rest) = parse_num(&chars).unwrap();
        assert!(rest.len() == 0);
        Ok(n)
    }
}

fn parse_num(chars: &[u8]) -> Option<(Number, &[u8])> {
    if chars[0] == b'[' {
        let (a, rest) = parse_num(&chars[1..]).unwrap();
        assert!(rest[0] == b',');
        let (b, rest) = parse_num(&rest[1..]).unwrap();
        assert!(rest[0] == b']');

        Some((Number::Box(Box::new(a), Box::new(b)), &rest[1..]))
    } else {
        let last_i = chars
            .iter()
            .enumerate()
            .take_while(|(_, ch)| ch.is_ascii_digit())
            .map(|(i, _)| i)
            .last()
            .unwrap();
        let s = chars[..=last_i]
            .iter()
            .map(|&b| b as char)
            .collect::<String>();
        let n = s.parse().unwrap();

        Some((Number::N(n), &chars[last_i + 1..]))
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Number::*;

        match self {
            N(n) => write!(f, "{}", n),
            Box(a, b) => write!(f, "[{}, {}]", a, b),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_num(a: Number, b: Number) -> Number {
        Number::Box(Box::new(a), Box::new(b))
    }

    fn new_pair(a: i64, b: i64) -> Number {
        Number::Box(Box::new(Number::N(a)), Box::new(Number::N(b)))
    }

    #[test]
    fn test_parse() {
        let s = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let n: Number = s.parse().unwrap();
        assert_eq!(
            n,
            new_num(
                new_num(
                    new_num(new_pair(1, 2), new_pair(3, 4),),
                    new_num(new_pair(5, 6), new_pair(7, 8),),
                ),
                Number::N(9),
            ),
        );
    }

    #[test]
    fn test_explode() {
        fn assert_explode(a: &str, b: &str) {
            let mut a: Number = a.parse().unwrap();
            let b: Number = b.parse().unwrap();

            a.explode(0);

            assert_eq!(a, b);
        }

        assert_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        assert_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        assert_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        assert_explode(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        assert_explode(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn test_split() {
        fn assert_split(from: &str, to: &str) {
            let mut from: Number = from.parse().unwrap();
            let to: Number = to.parse().unwrap();

            from.split();
            assert_eq!(from, to);
        }

        assert_split("10", "[5,5]");
        assert_split("11", "[5,6]");
        assert_split("12", "[6,6]");
    }

    #[test]
    fn test_add() {
        let a: Number = "[1,2]".parse().unwrap();
        let b: Number = "[[3,4],5]".parse().unwrap();
        let r: Number = "[[1,2],[[3,4],5]]".parse().unwrap();

        assert_eq!(a + b, r);
    }

    #[test]
    fn test_add_many() {
        let nums = "\
            [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]\
        ";
        let res = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

        let nums: Vec<Number> = nums.lines().map(str::trim).map(str::parse).collect::<Result<_, _>>().unwrap();
        let res: Number = res.parse().unwrap();

        assert_eq!( nums.into_iter().sum::<Number>(), res);
    }

    #[test]
    fn test_magnitude() {
        fn assert_mag(a: &str, expected: i64) {
            let a: Number = a.parse().unwrap();

            assert_eq!(a.magnitude(), expected);
        }

        assert_mag("[[1,2],[[3,4],5]]", 143);
        assert_mag("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        assert_mag("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        assert_mag("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        assert_mag("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        assert_mag("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488);

    }
}
