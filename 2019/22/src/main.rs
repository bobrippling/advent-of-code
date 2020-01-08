use std::fs;

/*macro_rules! deck {
    deck(n) {
    }
}*/

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Card(i32);

trait CopyRight {
    fn copy_right(&mut self, n: usize);
}

#[derive(Debug, Eq, PartialEq)]
struct Deck {
    // top: 0, bottom: n-1
    cards: Vec<Card>,
}

enum ShuffleEnt {
    Deal,
    Cut(isize),
    Increment(usize)
}

struct Shuffle(Vec<ShuffleEnt>);

impl Shuffle {
    fn new<I>(iter: I) -> Self
        where I: IntoIterator<Item = ShuffleEnt>
    {
        Self (iter.into_iter().collect())
    }
}

impl<T> CopyRight for Vec<T>
where T: Copy
{
    fn copy_right(&mut self, n: usize) {
        assert!(n > 0);

        let mut to = self.len() - 1;
        let mut from = to - n;

        loop {
            self[to] = self[from];

            if from == 0 {
                break;
            }
            if from == n {
                break;
            }

            to -= 1;
            from -= 1;
        }
    }
}

impl Deck {
    fn new(n: usize) -> Self {
        Self {
            cards: (0..n)
                .map(|i| Card(i as _))
                .collect(),
        }
    }

    fn deal_new(&mut self) {
        self.cards.reverse();
    }

    fn cut(&mut self, n: isize) {
        if n < 0 {
            let to_take = -n as usize;
            /*let mut bot_n = self.cards
                .iter()
                .rev()
                .take(to_take as _)
                .cloned();
                .collect::<Vec<_>>();

            self.cards.copy_right(to_take);
            bot_n.reverse();
            for (i, x) in bot_n.iter().enumerate() {
                self.cards[i] = *x;
            }*/

            let mut new = Vec::new();
            let l = self.cards.len();

            new.extend_from_slice(&self.cards[l - to_take .. l]);
            new.extend_from_slice(&self.cards[..l - to_take]);

            self.cards = new;

        } else {
            let top_n = self.cards
                .drain(0..n as _)
                .collect::<Vec<_>>();

            self.cards.extend(top_n.iter());
        }
    }

    fn increment(&mut self, n: usize) {
        let mut other = vec![Card(-1); self.cards.len()];

        for (from, to) in
            (0..other.len())
                .zip(
                    (0..).step_by(n).map(|n| n % self.cards.len())
                )
        {
            other[to] = self.cards[from];
        }

        self.cards = other;

        for card in &self.cards {
            assert!(card.0 != -1);
        }
    }

    fn apply(&mut self, shuffles: &Shuffle) {
        for shuf in &shuffles.0 {
            match shuf {
                ShuffleEnt::Deal => self.deal_new(),
                ShuffleEnt::Cut(c) => self.cut(*c),
                ShuffleEnt::Increment(i) => self.increment(*i),
            }
        }
    }
}

#[cfg(test)]
fn make_deck(ents: &[i32]) -> Deck {
    Deck {
        cards: ents.iter().map(|&i| Card(i)).collect(),
    }
}

fn parse(s: &str) -> Shuffle {
    Shuffle::new(
        s
            .split('\n')
            .map(|s| s.trim_start())
            .filter(|s| !s.is_empty())
            .map(|s| {
                if s.starts_with("cut ") {
                    let cut = s.get(4..).unwrap().parse().unwrap();
                    ShuffleEnt::Cut(cut)
                } else if s.starts_with("deal with increment ") {
                    let inc = s.get(20..).unwrap().parse().unwrap();
                    ShuffleEnt::Increment(inc)
                } else if s == "deal into new stack" {
                    ShuffleEnt::Deal
                } else {
                    panic!("unknown line: \"{}\"", s);
                }
            }))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shuffle = parse(&fs::read_to_string("./input")?);
    const N: usize = 10007;
    let mut deck = Deck::new(N);
    let req = 2019;

    deck.apply(&shuffle);

    /*
    println!(
        "cards 2017..2021: {:?}",
        &deck.cards[2017..2021]);

    // tried: 2070, 5064, 9083

    println!("card 2019: {:?}", deck.cards[2019]);
    */

    println!("{:?}", deck.cards.iter().position(|&Card(x)| x == 2019));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_right_short() {
        let mut a = vec![1, 2, 3];
        a.copy_right(1);
        assert_eq!(a, vec![1, 2, 2]);

        a[0] = 5;
        a.copy_right(2);
        assert_eq!(a, vec![5, 2, 5]);
    }

    #[test]
    fn copy_right_long() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7];
        a.copy_right(4);
        assert_eq!(a, vec![1, 2, 3, 4, 1, 2, 3]);
    }

    #[test]
    fn deal() {
        let mut deck = Deck::new(10);

        assert_eq!(
            deck,
            make_deck(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        );

        deck.deal_new();

        assert_eq!(
            deck,
            make_deck(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
        );
    }

    #[test]
    fn cut() {
        let mut deck = Deck::new(10);

        deck.cut(3);

        assert_eq!(
            deck,
            make_deck(&[3, 4, 5, 6, 7, 8, 9, 0, 1, 2]),
        );
    }

    #[test]
    fn cut_neg() {
        let mut deck = Deck::new(10);

        deck.cut(-4);

        assert_eq!(
            deck,
            make_deck(&[6, 7, 8, 9, 0, 1, 2, 3, 4, 5]),
        );
    }

    #[test]
    fn increment() {
        let mut deck = Deck::new(10);

        deck.increment(3);

        assert_eq!(
            deck,
            make_deck(&[0, 7, 4, 1, 8, 5, 2, 9, 6, 3]),
        );
    }

    fn assert_shuffle(shuffle: &str, expected: &Vec<i32>) {
        let expected = make_deck(expected);
        let shuffle = parse(shuffle);

        let mut deck = Deck::new(expected.cards.len());
        deck.apply(&shuffle);

        assert_eq!(deck, expected);
    }

    #[test]
    fn part1_eg1() {
        assert_shuffle(
            "
                deal with increment 7
                deal into new stack
                deal into new stack
            ",
            &vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn part1_eg2() {
        assert_shuffle(
            "
                cut 6
                deal with increment 7
                deal into new stack
            ",
            &vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn part1_eg3() {
        assert_shuffle(
            "
                deal with increment 7
                deal with increment 9
                cut -2
            ",
            &vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn part1_eg4() {
        assert_shuffle(
            "
                deal into new stack
                cut -2
                deal with increment 7
                cut 8
                cut -4
                deal with increment 7
                cut 3
                deal with increment 9
                deal with increment 3
                cut -1
            ",
            &vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
