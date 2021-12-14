use std::collections::HashMap;
use std::iter;
use std::ptr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input.txt")?.parse()?;

    if false {
        // some UB in here, causing later tests to fail
        println!("Part 1: {}", part1(&lines));
    }
    println!("Part 1: {} // fast", fast(&lines, 10));

    println!("Part 2: {}", part2(&lines));

    Ok(())
}

#[derive(Clone)]
struct Polymer {
    template: Template,
    rules: HashMap<(Letter, Letter), Letter>,
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Copy)]
struct Letter(u8);

#[derive(Clone)]
struct Template {
    head: *mut NodeBits,
}

struct NodeBits {
    elem: Letter,
    next: *mut NodeBits,
}

fn part1(input: &Polymer) -> u64 {
    let mut polymer = input.clone();

    for _ in 1..=10 {
        polymer.polymerise();
    }

    let mut counts = HashMap::<Letter, u64>::new();
    for &ch in polymer.template.letters() {
        *counts.entry(ch).or_default() += 1;
    }

    let most_common = counts.iter().max_by_key(|&(_, count)| count).unwrap();
    let least_common = counts.iter().min_by_key(|&(_, count)| count).unwrap();

    most_common.1 - least_common.1
}

fn part2(input: &Polymer) -> u64 {
    fast(input, 40)
}

fn fast(input: &Polymer, steps: usize) -> u64 {
    let mut polymer = input.to_pairs();
    let mut counts = HashMap::<Letter, u64>::new();

    for &letter in input.template.letters() {
        *counts.entry(letter).or_default() += 1;
    }

    for _ in 0..steps {
        for (pair, count) in polymer.clone() {
            if let Some(&letter) = input.rules.get(&pair) {
                *counts.entry(letter).or_default() += count;

                *polymer.get_mut(&pair).unwrap() -= count;

                for new in [(pair.0, letter), (letter, pair.1)] {
                    *polymer.entry(new).or_default() += count;
                }
            }
        }
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()

    // for ((a, b), n) in polymer {
    //     *counts.entry(a).or_default() += n;
    //     *counts.entry(b).or_default() += n;
    // }
    // for val in counts.values_mut() {
    //     *val /= 2;
    // }

    // let (&min_letter, &(mut min_n)) = counts.iter().min_by_key(|(_, &n)| n).unwrap();
    // let (&max_letter, &(mut max_n)) = counts.iter().max_by_key(|(_, &n)| n).unwrap();

    // let first = input.template.letters().next().unwrap();
    // let last = input.template.letters().last().unwrap();
    // let wrap_pair = [*first, *last];

    // println!("  first: {}, last: {}", first, last);

    // min_n += wrap_pair.contains(&min_letter) as u64;
    // max_n += wrap_pair.contains(&max_letter) as u64;

    // println!("  most: {}, least: {}", max_letter, min_letter);

    // max_n - min_n
}

impl Polymer {
    fn polymerise(&mut self) {
        let mut inserts: Vec<(*mut _, Letter)> = vec![];

        for (_i, (a, b, ptr)) in self.template.window_pairs().enumerate() {
            let pair = (*a, *b);
            if let Some(letter) = self.rules.get(&pair) {
                inserts.push((ptr, *letter))
            }
        }

        for (ptr, letter) in inserts.into_iter().rev() {
            insert_after(ptr, letter);
            // self.template.insert(i, letter);
        }
    }

    fn to_pairs(&self) -> HashMap<(Letter, Letter), u64> {
        let mut map = HashMap::default();

        for (&a, &b, _) in self.template.window_pairs() {
            *map.entry((a, b)).or_default() += 1;
        }

        map
    }
}

impl std::str::FromStr for Polymer {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("\n\n").collect();

        if let [template, rules_str] = parts[..] {
            let mut rules = HashMap::new();

            rules_str.lines().map(str::trim).for_each(|l| {
                let parts: Vec<_> = l.split(" -> ").collect();

                if let [from, to] = parts[..] {
                    let from: Vec<_> = from.bytes().collect();

                    if let [a, b] = from[..] {
                        rules.insert((Letter(a), Letter(b)), to.parse().unwrap());
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            });

            unsafe {
                let mut head = None;
                let mut cur: Option<*mut NodeBits> = None;

                for l in template.bytes().map(Letter) {
                    let next = new_letter(l);

                    match cur {
                        Some(bits) => {
                            assert!((*bits).next.is_null());
                            assert!(head.is_some());

                            (*bits).next = next;
                            cur = Some(next);
                        }
                        None => {
                            assert!(head.is_none());
                            head = Some(next);
                            cur = Some(next);
                        }
                    }
                }

                Ok(Self {
                    template: Template {
                        head: head.unwrap(),
                    },
                    rules,
                })
            }
        } else {
            Err("wrong paragraph count")
        }
    }
}

fn new_letter(l: Letter) -> *mut NodeBits {
    Box::leak(Box::new(NodeBits {
        elem: l,
        next: ptr::null_mut(),
    })) as *mut _
}

impl Template {
    // fn len(&self) -> usize {
    //     self.letters().count()
    // }

    fn window_pairs(&self) -> impl Iterator<Item = (&Letter, &Letter, *mut NodeBits)> + '_ {
        let mut prev: Option<*mut NodeBits> = None;
        let mut current = self.head;

        iter::from_fn(move || loop {
            if current.is_null() {
                return None;
            }

            match prev {
                Some(prev_ptr) => {
                    unsafe {
                        prev = Some(current);

                        let cur = current;
                        current = (*current).next;

                        break Some((&(*prev_ptr).elem, &(*cur).elem, prev_ptr));
                    };
                }
                None => {
                    prev = Some(current);
                    unsafe {
                        current = (*current).next;
                    }
                }
            }
        })
    }

    fn letters(&self) -> impl Iterator<Item = &Letter> + '_ {
        let mut current = self.head;

        iter::from_fn(move || {
            if current.is_null() {
                None
            } else {
                unsafe {
                    let l = &(*current).elem;
                    current = (*current).next;
                    Some(l)
                }
            }
        })
    }

    // fn insert(&mut self, i: usize, letter: Letter) {
    //     unsafe {
    //         let mut current = self.head;

    //         for _ in 0..i {
    //             assert!(!current.is_null());
    //             current = (*current).next;
    //         }

    //         insert_after(current, letter);
    //     }
    // }
}

fn insert_after(p: *mut NodeBits, letter: Letter) -> *mut NodeBits {
    let new = new_letter(letter);

    unsafe {
        let next = (*p).next;
        (*p).next = new;
        (*new).next = next;
    }

    new
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.letters() {
            write!(f, "{}", l.0 as char)?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

impl std::str::FromStr for Letter {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<_> = s.bytes().collect();

        if let [ch] = chars[..] {
            Ok(Self(ch))
        } else {
            Err("wrong char count")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG: &'static str = "\
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C\
    ";

    #[test]
    fn test_part1() {
        let lines = EG.parse().unwrap();
        assert_eq!(part1(&lines), 1588);
    }

    #[test]
    fn test_part1_fast() {
        let lines = EG.parse().unwrap();
        assert_eq!(fast(&lines, 10), 1588);
    }

    #[test]
    fn test_part2() {
        let lines = EG.parse().unwrap();
        assert_eq!(part2(&lines), 2188189693529);
    }

    #[test]
    fn letters() {
        let template = {
            let head = new_letter(Letter(b'a'));
            insert_after(insert_after(head, Letter(b'b')), Letter(b'c'));
            Template { head }
        };

        assert_eq!(
            template.letters().collect::<Vec<_>>(),
            vec![&Letter(b'a'), &Letter(b'b'), &Letter(b'c'),]
        );
    }

    #[test]
    fn window_pairs() {
        let a = new_letter(Letter(b'a'));
        let b = new_letter(Letter(b'b'));
        let c = new_letter(Letter(b'c'));

        unsafe {
            (*a).next = b;
            (*b).next = c;
        }
        let template = Template { head: a };

        assert_eq!(
            template.window_pairs().collect::<Vec<_>>(),
            vec![
                (&Letter(b'a'), &Letter(b'b'), a),
                (&Letter(b'b'), &Letter(b'c'), b),
            ]
        );
    }
}
