use std::fs;

const DEBUG: bool = false;
const PROGRESS: bool = false;

fn main() -> Result<(), Box<dyn (std::error::Error)>> {
    let s = fs::read_to_string("./input.txt").unwrap();

    let cups: Cups = s.parse()?;

    println!("Part 1: {}", part1(cups.clone(), 100));
    println!("Part 2: {}", part2(cups));

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cups {
    cups: Vec<Entry>,
    first: Cup,
    cur: Cup,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cup(usize);

impl std::fmt::Debug for Cup {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "c{}", self.0)
    }
}

impl std::fmt::Display for Cup {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)
    }
}

impl std::ops::Index<Cup> for Cups {
    type Output = Entry;

    fn index(&self, cup: Cup) -> &Self::Output {
        assert!(cup.0 > 0, "invalid cup");
        &self.cups[cup.0 - 1]
    }
}

impl std::ops::IndexMut<Cup> for Cups {
    fn index_mut(&mut self, cup: Cup) -> &mut Self::Output {
        assert!(cup.0 > 0, "invalid cup");
        &mut self.cups[cup.0 - 1]
    }
}

impl std::str::FromStr for Cups {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars()
            .filter(|ch| ch.is_numeric())
            .map(|ch| ch as u8 - '0' as u8)
            .map(|x| Cup(x as usize))
            .collect::<Vec<_>>();

        let first = chars[0];
        let last = chars[chars.len()-1];
        let n = chars.len();

        let mut cups = Cups::new(first, n);

        for i in 0..chars.len()-1 {
            let cup = chars[i];
            let next = chars[i+1];

            cups[cup].next = next;
            cups[next].prev = cup;
        }
        cups[last].next = first;
        cups[first].prev = last;

        cups.assert_valid();

        Ok(cups)
    }
}

#[test]
fn test_parse() {
    let input = "4132";
    let cups: Cups = input.parse().unwrap();

    assert_eq!(
        cups,
        Cups {
            cups: vec![
                Entry { next: Cup(3), prev: Cup(4) }, // cup 1
                Entry { next: Cup(4), prev: Cup(3) }, // cup 2
                Entry { next: Cup(2), prev: Cup(1) }, // cup 3
                Entry { next: Cup(1), prev: Cup(2) }, // cup 4
            ],
            first: Cup(4),
            cur: Cup(4),
        },
    );
}

struct CupIter<'a> {
    cups: &'a Cups,
    at: Cup,
    done: usize,
}

impl Cups {
    fn iter_from(&self, from: Cup) -> CupIter<'_> {
        CupIter {
            cups: self,
            at: from,
            done: 0,
        }
    }
}

impl Iterator for CupIter<'_> {
    type Item = Cup;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done == self.cups.cups.len() {
            return None;
        }

        self.done += 1;
        let current_cup = self.at;
        self.at = self.cups[current_cup].next;
        Some(current_cup)
    }
}

#[test]
fn test_iter() {
    let cups = Cups {
        cups: vec![
            Entry { next: Cup(3), prev: Cup(4) }, // third (1)
            Entry { next: Cup(4), prev: Cup(3) }, // first (2)
            Entry { next: Cup(2), prev: Cup(1) }, // fourth (3)
            Entry { next: Cup(1), prev: Cup(2) }, // second (4)
        ],
        first: Cup(2),
        cur: Cup(2),
    };
    cups.assert_valid();

    assert_eq!(
        cups.iter_from(Cup(2)).map(cup_to_char).collect::<String>(),
        "2413",
    );
}

fn cup_to_char(cup: Cup) -> char {
    (cup.0 as u8 + '0' as u8) as char
}

impl Cups {
    fn do_move(&mut self, turn: usize) {
        let cur_label = self.cur;

        let a = self[cur_label].next;
        let b = self[a].next;
        let c = self[b].next;

        let mut dest = self.find_destination(cur_label);
        while dest == a || dest == b || dest == c {
            dest = self.find_destination(dest);
        }

        if DEBUG {
            println!("-- move {} --", turn);
            println!("cups: {}", self);
            println!("pick up: {}, {}, {}", a, b, c);
            println!("destination: {}\n", dest);
        }

        for &cup in &[c, b, a] {
            self.put_after(dest, cup);
        }

        self.cur = self[self.cur].next;
    }

    fn put_after(&mut self, dest: Cup, cup: Cup) {
        self.remove(cup);

        let old_next = self[dest].next;

        self[dest].next = cup;
        self[cup].prev = dest;

        self[cup].next = old_next;
        self[old_next].prev = cup;

        if DEBUG {
            self.assert_valid();
        }
    }

    fn remove(&mut self, cup: Cup) {
        let Entry { next, prev } = self[cup];

        self[next].prev = prev;
        self[prev].next = next;
    }

    fn find_destination(&self, label: Cup) -> Cup {
        if label.0 == 1 {
            Cup(self.cups.len())
        } else {
            Cup(label.0 - 1)
        }
    }
}

impl Cups {
    fn new(first: Cup, n: usize) -> Self {
        Self {
            cups: vec![Entry::zero(); n],
            first,
            cur: first,
        }
    }

    fn assert_valid(&self) {
        let total_next: usize = self.cups
            .iter()
            .map(|Entry { next: Cup(i), .. }| i)
            .sum();

        let total_prev: usize = self.cups
            .iter()
            .map(|Entry { prev: Cup(i), .. }| i)
            .sum();

        //println!("total: {}, cups: {:?}", total, self);

        let n = self.cups.len();
        assert_eq!(total_next, n * (n + 1) / 2, "invalid .next links");
        assert_eq!(total_prev, n * (n + 1) / 2, "invalid .prev links");
    }
}

impl std::fmt::Display for Cups {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut cur = self.first;

        loop {
            if cur == self.cur {
                write!(fmt, "({})", cur.0)?;
            } else {
                write!(fmt, " {} ", cur.0)?;
            }

            cur = self[cur].next;
            if cur == self.first {
                break;
            }
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Entry {
    next: Cup,
    prev: Cup,
}

impl Entry {
    fn zero() -> Self {
        Self {
            next: Cup(0),
            prev: Cup(0),
        }
    }
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "next={:?} prev={:?}", self.next, self.prev)
    }
}

fn part1(mut cups: Cups, moves: usize) -> String {
    for turn in 1..=moves {
        cups.do_move(turn);
    }

    cups.iter_from(Cup(1))
        .skip(1)
        .map(cup_to_char)
        .collect()
}

#[test]
fn test_part1() {
    let input = "389125467";

    assert_eq!(part1(input.parse().unwrap(), 10), "92658374");
}

fn part2(cups: Cups) -> usize {
    let highest = Cup(cups.cups.len());
    let first_new = Cup(highest.0 + 1);

    let mut vec = cups.cups;

    (first_new.0..)
        .map(Cup)
        .take(1_000_000 - vec.len())
        .for_each(|cup| {
            vec.push(Entry {
                next: Cup(cup.0 + 1),
                prev: Cup(cup.0 - 1),
            });
        });

    assert_eq!(vec.len(), 1_000_000);

    let mut cups = Cups {
        cups: vec,
        first: cups.first,
        cur: cups.cur,
    };

    // link up the tail, and the last one
    {
        let last = cups[cups.first].prev;

        cups[last].next = first_new;
        cups[first_new].prev = last;

        let first = cups.first;
        let last = Cup(1_000_000);

        cups[last].next = first;
        cups[first].prev = last;

        cups.assert_valid();
    }

    let moves = 10_000_000;
    for turn in 1..=moves {
        cups.do_move(turn);
        if PROGRESS && turn % 10_000 == 0 {
            print!("\x1b[K{}%...\r", 100.0 * turn as f64 / moves as f64);
        }
    }
    if PROGRESS {
        print!("\x1b[K\r");
    }
    cups.assert_valid();

    cups.iter_from(Cup(1))
        .skip(1)
        .take(2)
        .fold(1, |t, Cup(x)| t * x)
}

#[test]
fn test_part2() {
    let input = "389125467";

    assert_eq!(part2(input.parse().unwrap()), 149245887792);
}
