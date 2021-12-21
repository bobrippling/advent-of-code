use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
struct Track {
    players: [(Pos, u64); 2],
}

#[derive(Clone, Default, PartialEq, Eq, Hash)]
struct Pos(u32);

struct DeterministicDie {
    i: u32,
    roll_count: u32,
}

fn part1(track: &Track) -> u64 {
    let mut die = DeterministicDie::new();
    let mut track = track.clone();

    let score_limit = 1000;

    let loser = loop {
        track.move_player(1, die.roll3());
        if let Some(lose) = track.losing_score(score_limit) {
            break lose;
        }
        track.move_player(2, die.roll3());
        if let Some(lose) = track.losing_score(score_limit) {
            break lose;
        }
    };

    loser * die.roll_count as u64
}

struct Tracks {
    track: Track,
}

fn part2(track: &Track) -> u64 {
    let tracks = Tracks::from(track.clone());

    let wins = tracks.calc_wins(1, &mut Default::default());
    let (wins_a, wins_b) = wins;

    wins_a.max(wins_b)
}

struct Dirac {
    roll: u8,
    count: u8,
}
// Generated from perl:
// for $x (1..3) {
//   for $y (1..3) {
//     for $z (1..3) {
//       $total{$x + $y + $z}++;
//     }
//   }
// }
// for(keys %total){
//   print "roll: $_, count: $total{$_}\n";
// }
static DIRAC_ROLLS: [Dirac; 7] = [
    Dirac { roll: 3, count: 1 },
    Dirac { roll: 4, count: 3 },
    Dirac { roll: 5, count: 6 },
    Dirac { roll: 6, count: 7 },
    Dirac { roll: 7, count: 6 },
    Dirac { roll: 8, count: 3 },
    Dirac { roll: 9, count: 1 },
];

impl Tracks {
    fn calc_wins(&self, turn: u8, cache: &mut HashMap<(Track, u8), (u64, u64)>) -> (u64, u64) {
        if let Some(subwins) = cache.get(&(self.track.clone(), turn)) {
            return *subwins;
        }

        let mut wins = (0, 0);
        for &Dirac { roll, count } in &DIRAC_ROLLS {
            let mut universe = self.track.clone();

            universe.move_player(turn as _, roll as _);
            if let Some(winner) = universe.winner(21) {
                if winner == 1 {
                    wins.0 += count as u64;
                }
            } else {
                let sub = Self::from(universe);
                let subwins = sub.calc_wins(if turn == 1 { 2 } else { 1 }, cache);
                wins.0 += count as u64 * subwins.1;
                wins.1 += count as u64 * subwins.0;
            }
        }

        cache.insert((self.track.clone(), turn), wins);

        wins
    }
}

impl From<Track> for Tracks {
    fn from(track: Track) -> Self {
        Self { track }
    }
}

impl Track {
    fn move_player(&mut self, i: usize, inc: u32) {
        assert!(matches!(i, 1 | 2));

        let player = &mut self.players[i - 1];

        player.0.inc(inc);

        let pos = player.0.as_int();
        player.1 += pos;
    }

    fn losing_score(&self, score_limit: u64) -> Option<u64> {
        for (i, p) in self.players.iter().enumerate() {
            if p.1 >= score_limit {
                let other = if i == 0 { 1 } else { 0 };
                return Some(self.players[other].1);
            }
        }
        None
    }

    fn winner(&self, score_limit: u64) -> Option<usize> {
        for (i, p) in self.players.iter().enumerate() {
            if p.1 >= score_limit {
                return Some(i + 1);
            }
        }
        None
    }
}

impl Pos {
    fn new(one_based: u32) -> Self {
        Self(one_based - 1)
    }

    fn inc(&mut self, n: u32) {
        self.0 = (self.0 + n) % 10;
    }

    fn as_int(&self) -> u64 {
        (self.0 + 1) as _
    }
}

impl DeterministicDie {
    fn new() -> Self {
        Self {
            i: 1,
            roll_count: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        // 1..=100
        let r = self.i;
        if self.i == 100 {
            self.i = 1;
        } else {
            self.i += 1;
        }
        self.roll_count += 1;
        r
    }

    fn roll3(&mut self) -> u32 {
        (0..3).map(|_| self.roll()).sum()
    }
}

impl std::str::FromStr for Track {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut track = Self::default();

        s.lines().for_each(|l| {
            let b = l.trim().bytes().collect::<Vec<_>>();
            let p = b[7] - b'0';
            let pos = b[28] - b'0';
            match p {
                1 | 2 => {
                    track.players[p as usize - 1].0 = Pos::new(pos as _);
                }
                _ => panic!(),
            }
        });

        Ok(track)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_die() {
        let mut die = DeterministicDie::new();

        assert_eq!(die.roll(), 1);
        assert_eq!(die.roll3(), 2 + 3 + 4);
    }

    #[test]
    fn test_part1() {
        let input = EG.parse().unwrap();

        assert_eq!(part1(&input), 739785);
    }

    #[test]
    fn test_part2() {
        let input = EG.parse().unwrap();

        assert_eq!(part2(&input), 444356092776315);
    }

    static EG: &'static str = "Player 1 starting position: 4
        Player 2 starting position: 8\
    ";
}
