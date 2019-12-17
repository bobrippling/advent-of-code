use std::fs;
use std::io::Write;

type N = i64;

static BASE_PATTERN: &[N] = &[0, 1, 0, -1];
const DEBUG: bool = false;

struct RepeatIter<'a, X> {
    repeats: usize,
    this_repeat: usize,
    other: &'a mut dyn Iterator<Item = X>,
    saved: Option<X>,
}

impl<'a, X> RepeatIter<'a, X>
where X: Copy
{
    fn new(o: usize, other: &'a mut dyn Iterator<Item = X>) -> Self {
        Self {
            repeats: o,
            this_repeat: 0,
            other,
            saved: None,
        }
    }
}

impl<'a, X> Iterator for RepeatIter<'a, X>
    where X: Copy
{
    type Item = X;

    fn next(&mut self) -> Option<Self::Item> {
        match self.saved {
            None => {
                // init
                match self.other.next() {
                    Some(x) => {
                        self.saved = Some(x);
                        self.this_repeat = self.repeats;
                        self.saved
                    },
                    None => {
                        None
                    }
                }
            },
            Some(n) => {
                if self.this_repeat == 0 {
                    match self.other.next() {
                        None => {
                            None
                        },
                        Some(x) => {
                            self.saved = Some(x);
                            self.this_repeat = self.repeats;
                            self.saved
                        }
                    }
                } else {
                    self.this_repeat -= 1;
                    Some(n)
                }
            },
        }
    }
}

/*
trait Repeatable<I> {
    fn repeated<'a>(&'a mut self, n: usize) -> RepeatIter<'a, I>;
}

impl<I> Repeatable<I> for dyn Iterator<Item = N>
where Self == I
{
    fn repeated(&mut self, n: usize) -> RepeatIter<I> {
        RepeatIter {
            repeats: n,
            this_repeat: 0,
            other: self,
            saved: None,
        }
    }
}
*/

fn clamp(n: N) -> N {
    n.abs() % 10
}

fn phase(/*_phase: i32, */input: &[N], pattern: &[N]) -> Vec<N> {
    let mut out = Vec::new();

    for output_idx in 0..input.len() {
        let mut simple = pattern
            .iter()
            .cloned()
            .cycle();
        let this_pattern = RepeatIter::new(
            output_idx,
            &mut simple
        ).skip(1);

        if DEBUG {
            println!("--- {} ---", output_idx);
        }
        let sum = (0..input.len())
            .zip(this_pattern)
            .map(|(i, pat)| {
                if DEBUG {
                    println!("{} * {}", input[i], pat);
                }
                input[i] * pat
            })
            .sum();

        if DEBUG {
            println!("summed: {}", sum);
        }
        out.push(clamp(sum));
    }

    out
}

/*
fn fft(nums: &[N]) -> Vec<N> {
    //let pattern = [ ];

    //phase(0, nums, );
    vec![]
}
*/

fn parse(s: &str) -> Vec<N> {
    let ns = s.chars()
        .map(|c| {
            c.to_string().parse()
        })
        .collect::<Result<_, _>>()
        .expect("couldn't parse");
    ns
}

#[allow(dead_code)]
fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input")?;
    let s = s.trim();
    let mut a = parse(&s);
    for _ in 0..100 {
        a = phase(&a, &BASE_PATTERN);
    }

    /*
    let fin = a.iter()
        .take(8)
        .fold(|r, x|
    println!("{}", */
    for x in a.iter().take(8) {
        print!("{} ", x);
    }
    println!();

    Ok(())
}

fn save(a: &[N]) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = std::fs::File::create("vec")?;

    for &x in a {
        writeln!(f, "{}", x)?;
    }

    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input")?;
    let s = s.trim();

    let first7 = &s[0 .. 8];
    let n: usize = first7.parse()?;
    //let s = &s[8..];

    let src = parse(&s);
    let mut a = Vec::new();
    a.extend(src.iter().cycle().take(10000 * src.len()));

    for i in 0..100 {
        a = phase(&a, &BASE_PATTERN);
        println!("{}", i);
    }

    for x in a.iter().skip(n).take(8) {
        print!("{}", x);
    }
    println!();

    save(&a)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //part1()?;
    //part2()?;
    eg()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let x = parse("01029498");

        assert_eq!(
            x,
            vec![0,1,0,2,9,4,9,8]);
    }

    #[test]
    fn test_repeat_iter() {
        let a: &[N] = &[1, 2, 3];
        let mut iter = a.iter().cloned();
        let mut repeat = RepeatIter::new(
            0, &mut iter,
        );

        assert_eq!(repeat.next(), Some(1));
        assert_eq!(repeat.next(), Some(2));
        assert_eq!(repeat.next(), Some(3));
        assert_eq!(repeat.next(), None);
    }

    #[test]
    fn test_repeat_iter_cycle() {
        let a: &[N] = &[1, 2, 3];
        let mut iter = a.iter().cloned().cycle();
        let mut repeat = RepeatIter::new(
            0, &mut iter,
        );

        assert_eq!(repeat.next(), Some(1));
        assert_eq!(repeat.next(), Some(2));
        assert_eq!(repeat.next(), Some(3));
        assert_eq!(repeat.next(), Some(1));
        assert_eq!(repeat.next(), Some(2));
        assert_eq!(repeat.next(), Some(3));
        assert_eq!(repeat.next(), Some(1));
    }

    #[test]
    fn test_repeat_iter_step() {
        let a: &[N] = &[1, 2, 3];
        let mut iter = a.iter().cloned();
        let mut repeat = RepeatIter::new(
            2, &mut iter,
        );

        assert_eq!(repeat.next(), Some(1));
        assert_eq!(repeat.next(), Some(1));
        assert_eq!(repeat.next(), Some(1));
        assert_eq!(repeat.next(), Some(2));
        assert_eq!(repeat.next(), Some(2));
        assert_eq!(repeat.next(), Some(2));
        assert_eq!(repeat.next(), Some(3));
        assert_eq!(repeat.next(), Some(3));
        assert_eq!(repeat.next(), Some(3));
        assert_eq!(repeat.next(), None);
    }

    /*
    #[test]
    fn test_day16_simple() {
        let input = [9, 8, 7, 6, 5];
        let pattern = [1,2,3];

        assert_eq!(
            phase(&input, &pattern)[0],
            clamp(9*1 + 8*2 + 7*3 + 6*1 + 5*2),
            );
    }
    */

    #[test]
    fn test_day16_eg1() {
        let input = parse("12345678");
        let mut x = input;

        let expected1 = parse("48226158");
        x = phase(&x, BASE_PATTERN);
        assert_eq!(&x, &expected1);

        let expected2 = parse("34040438");
        x = phase(&x, BASE_PATTERN);
        assert_eq!(&x, &expected2);

        let expected3 = parse("03415518");
        x = phase(&x, BASE_PATTERN);
        assert_eq!(&x, &expected3);

        let expected4 = parse("01029498");
        x = phase(&x, BASE_PATTERN);
        assert_eq!(&x, &expected4);
    }
}
