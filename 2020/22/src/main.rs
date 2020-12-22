use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let s = fs::read_to_string("./input.txt").unwrap();

    let decks = parse(&s);
    println!("Part 1: {}", part1(&mut decks.clone()));

    println!("Part 2: {}", part2(&mut decks.clone()));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Decks {
    me: Deck,
    op: Deck,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Deck {
    cards: VecDeque<u32>,
}

fn parse(s: &str) -> Decks {
    let mut decks = s.split("\n\n")
        .map(|lines| {
            let mut iter = lines.split('\n');
            assert!(iter.next().unwrap().starts_with("Player "));

            Deck { cards: iter.filter(|l| !l.is_empty()).map(str::parse).map(Result::unwrap).collect() }
        })
        .collect::<Vec<_>>();

    assert_eq!(decks.len(), 2);
    Decks {
        op: decks.pop().unwrap(),
        me: decks.pop().unwrap(),
    }
}

fn part1(decks: &mut Decks) -> usize {
    while decks.both_have_cards() {
        decks.round();
    }

    decks.winner_score()
}

fn part2(decks: &mut Decks) -> usize {
    decks.recursive_combat();
    decks.winner_score()
}

impl Deck {
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Win {
    P1, P2,
}

impl Decks {
    fn recursive_combat(&mut self) -> Win {
        let mut seen = HashSet::<Decks>::new();

        loop {
            if seen.contains(&self) {
                break Win::P1;
            }
            seen.insert(self.clone());

            let (m, o) = (self.me.cards.pop_front().unwrap(), self.op.cards.pop_front().unwrap());

            let this_winner = if self.me.cards.len() >= m as _ && self.op.cards.len() >= o as _ {
                let me_copy = self.me.cards.iter().cloned().take(m as usize).collect();
                let op_copy = self.op.cards.iter().cloned().take(o as usize).collect();

                let mut sub = Decks {
                    me: Deck { cards: me_copy },
                    op: Deck { cards: op_copy },
                };

                sub.recursive_combat()

            } else {
                if m > o {
                    Win::P1
                } else if o > m {
                    Win::P2
                } else {
                    panic!();
                }
            };

            match this_winner {
                Win::P1 => {
                    self.me.cards.push_back(m);
                    self.me.cards.push_back(o);
                }
                Win::P2 => {
                    self.op.cards.push_back(o);
                    self.op.cards.push_back(m);
                }
            }

            if self.me.is_empty() {
                break Win::P2;
            } else if self.op.is_empty() {
                break Win::P1;
            }
        }
    }

    fn both_have_cards(&self) -> bool {
        !self.me.is_empty() && !self.op.is_empty()
    }

    fn round(&mut self) {
        let (m, o) = (self.me.cards.pop_front().unwrap(), self.op.cards.pop_front().unwrap());

        if m > o {
            self.me.cards.push_back(m);
            self.me.cards.push_back(o);
        } else if o > m {
            self.op.cards.push_back(o);
            self.op.cards.push_back(m);
        } else {
            panic!();
        }
    }

    fn winner_score(&self) -> usize {
        if self.me.is_empty() {
            winner_score_of(&self.op)
        } else if self.op.is_empty() {
            winner_score_of(&self.me)
        } else {
            panic!();
        }
    }
}

fn winner_score_of(deck: &Deck) -> usize {
    deck.cards.iter()
        .rev()
        .cloned()
        .enumerate()
        .map(|(i, c)| {
            (i + 1) * (c as usize)
        })
        .sum()
}

#[test]
fn test_part1() {
    let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    let mut decks = parse(&input);

    assert_eq!(part1(&mut decks), 306);
}

#[test]
fn test_part2_infinite() {
    let input = "Player 1:
43
19

Player 2:
2
29
14";

    let mut decks = parse(&input);

    assert_eq!(decks.recursive_combat(), Win::P1);
}

#[test]
fn test_part2_recurse() {
    let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    let mut decks = parse(&input);

    assert_eq!(decks.recursive_combat(), Win::P2);

    assert_eq!(decks.winner_score(), 291);
}
