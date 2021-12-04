#[derive(Clone)]
struct Bingo {
    turns: Vec<u32>,
    cards: Vec<Card>,
    turn: usize,
}

#[derive(Clone)]
struct Card {
    rows: Vec<Vec<(u32, bool)>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bingo = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&bingo));
    println!("Part 2: {}", part2(&bingo));

    Ok(())
}

fn part1(bingo: &Bingo) -> usize {
    let mut bingo = bingo.clone();

    let card = loop {
        if let Some(card) = bingo.winning_card() {
            break card;
        }

        bingo.turn();
    };

    // println!("winning card:\n{:?}", card);

    card.winning_score() * bingo.latest_turn()
}

fn part2(bingo: &Bingo) -> usize {
    let mut bingo = bingo.clone();

    let card = loop {
        let index = bingo.sole_remaining_card_index();

        bingo.turn();

        if let Some(index) = index {
            if bingo.cards[index].has_won() {
                break &bingo.cards[index];
            }
        }
    };

    card.winning_score() * bingo.latest_turn()
}

impl Bingo {
    fn turn(&mut self) {
        let n = self.turns[self.turn];
        self.turn += 1;

        for c in &mut self.cards {
            c.mark(n);
        }
    }

    fn winning_card(&self) -> Option<&Card> {
        self.cards.iter().filter(|c| c.has_won()).next()
    }

    fn sole_remaining_card_index(&self) -> Option<usize> {
        let remaining: Vec<_> = self
            .cards
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.has_won())
            .map(|(i, _)| i)
            .collect();

        if let [single] = remaining[..] {
            Some(single)
        } else {
            None
        }
    }

    fn latest_turn(&self) -> usize {
        self.turns[self.turn - 1] as usize
    }
}

impl std::fmt::Debug for Bingo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Turns: {:?}", self.turns)?;
        writeln!(f, "Cards:")?;
        for c in self.cards.iter() {
            writeln!(f, "{:?}", c)?;
        }
        Ok(())
    }
}

impl Card {
    fn mark(&mut self, n: u32) {
        for row in &mut self.rows {
            for (x, mark) in row {
                if *x == n {
                    *mark = true;
                }
            }
        }
    }

    fn unmarked_nums(&self) -> impl Iterator<Item = u32> + '_ {
        self.rows
            .iter()
            .flat_map(|row| row.iter().filter(|&(_, mark)| !*mark).map(|&(n, _)| n))
    }

    fn winning_score(&self) -> usize {
        self.unmarked_nums().map(|n| n as usize).sum()
    }

    fn has_won(&self) -> bool {
        let marked_row = self
            .rows
            .iter()
            .any(|row| row.iter().all(|(_, mark)| *mark));

        if marked_row {
            return true;
        }

        self.cols().any(|col| col.iter().all(|(_, mark)| *mark))
    }

    fn cols(&self) -> impl Iterator<Item = Vec<(u32, bool)>> + '_ {
        let n = self.rows[0].len();

        (0..n).map(|col_index| {
            let mut v = Vec::new();

            for row in &self.rows {
                v.push(row[col_index]);
            }

            v
        })
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for &(n, mark) in row {
                let marker = if mark { '-' } else { ' ' };
                let space = if n < 10 { " " } else { "" };
                write!(f, " {}{}{}{} ", marker, space, n, marker)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::str::FromStr for Bingo {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut paragraphs = s.split("\n\n");

        let turns = paragraphs
            .next()
            .ok_or("not enough lines")?
            .split(',')
            .map(|s| s.parse().map_err(|_| "couldn't parse number"))
            .collect::<Result<Vec<_>, _>>()?;

        let cards = paragraphs.map(str::parse).collect::<Result<Vec<_>, _>>()?;

        Ok(Bingo {
            turns,
            cards,
            turn: 0,
        })
    }
}

impl std::str::FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Card {
            rows: s
                .lines()
                .map(|line| {
                    let row: Vec<_> = line
                        .split(' ')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(|s| {
                            s.parse()
                                .map_err(|_| "couldn't parse number")
                                .map(|n| (n, false))
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    assert!(!row.is_empty());

                    Ok(row)
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7\
        ";

        let bingo = input.parse().unwrap();
        assert_eq!(part1(&bingo), 4512);
    }

    #[test]
    fn test_part2() {
        let input = "\
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7\
        ";

        let bingo = input.parse().unwrap();
        assert_eq!(part2(&bingo), 1924);
    }
}
