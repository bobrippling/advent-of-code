use std::fs;
use std::convert::TryFrom;
use std::collections::HashSet;
use std::collections::HashMap;

#[cfg(feature = "show-steps")]
use std::io::{self, BufWriter, Write};

type SeatMap = HashMap<Pos, Seat>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;

    let (seats, max) = parse(&s)?;

    let occupied_after = part1(&seats, &max);
    println!("Part 1: {}", occupied_after);

    let occupied_after = part2(&seats, &max);
    println!("Part 2: {}", occupied_after);

    Ok(())
}

fn part1(seats: &SeatMap, max: &Pos) -> usize {
    let rest_state = game_of_life(
        seats,
        max,
        part1_iteration);

    count_occupied(&rest_state)
}

fn part1_iteration(seat: Seat, pos: &Pos, seats: &SeatMap) -> Option<Seat> {
    match seat {
        Seat::Empty if count_adjacent(seats, &pos) == 0 => {
            Some(Seat::Occupied)
        }

        Seat::Occupied if count_adjacent(seats, &pos) >= 4 => {
            Some(Seat::Empty)
        }

        _ => None
    }
}

fn game_of_life(
    seats: &SeatMap,
    max: &Pos,
    next_state: fn(Seat, &Pos, &SeatMap) -> Option<Seat>,
) -> SeatMap {
    let mut seats: SeatMap = (*seats).clone();
    let mut seen = HashSet::<String>::new();

    loop {
        seats = game_of_life_singlestep(&seats, next_state);

        let repr = gen_str(&seats, max);
        if seen.contains(&repr) {
            break seats;
        }

        #[cfg(feature = "show-steps")]
        {
            let mut w = BufWriter::new(io::stdout());
            write!(w, "iter:").unwrap();
            repr
                .chars()
                .enumerate()
                .for_each(|(i, ch)| {
                    if i % max.x == 0 {
                        write!(w, "\n").unwrap();
                    }
                    write!(w, "{}", ch).unwrap();
                });
            write!(w, "\n").unwrap();
        }

        seen.insert(repr);
    }
}

fn game_of_life_singlestep(
    seats: &SeatMap,
    next_state: fn(Seat, &Pos, &SeatMap) -> Option<Seat>,
) -> SeatMap {
    let mut new = seats.clone();

    for (&pos, &seat) in seats {
        if let Some(seat) = next_state(seat, &pos, &seats) {
            new.insert(pos, seat);
        }
    }

    new
}

fn count_adjacent(seats: &SeatMap, pos: &Pos) -> usize {
    let x = pos.x as isize;
    let y = pos.y as isize;

    [
        (x - 1, y - 1),
        (x    , y - 1),
        (x + 1, y - 1),
        (x - 1, y    ),
        // <current-seat>
        (x + 1, y    ),
        (x - 1, y + 1),
        (x    , y + 1),
        (x + 1, y + 1),
    ]
        .iter()
        .cloned()
        .map(|(x, y)| Pos { x: x as _, y: y as _ })
        .filter_map(|pos| seats.get(&pos).map(|s| *s))
        .filter(|&seat| seat == Seat::Occupied)
        .count()
}

fn gen_str(seats: &SeatMap, max: &Pos) -> String {
    CombinationsIter::new(0..max.x, 0..max.y)
        .map(|(x, y)| Pos { x, y })
        .map(|pos| seats.get(&pos).unwrap())
        .cloned()
        .map(<Seat as Into<char>>::into)
        .collect::<String>()
}

struct CombinationsIter<I0, I1>
where
    I0: Iterator,
{
    iter0: I0,
    iter1: I1,
    iter1_clone: I1,

    temp0: Option<<I0 as Iterator>::Item>,
}

impl<I0, I1> CombinationsIter<I0, I1>
where
    I0: Iterator,
    I1: Clone,
{
    fn new(mut iter0: I0, iter1: I1) -> Self {
        Self {
            temp0: iter0.next(),
            iter0,
            iter1: iter1.clone(),
            iter1_clone: iter1,
        }
    }
}

impl<I0, I1> Iterator for CombinationsIter<I0, I1>
where
    I0: Iterator,
    I1: Iterator + Clone,
    <I0 as Iterator>::Item: Copy
{
    type Item = (<I0 as Iterator>::Item, <I1 as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.temp0 {
                Some(a) => {
                    match self.iter1.next() {
                        Some(b) => {
                            break Some((a, b));
                        }
                        None => {
                            self.temp0 = self.iter0.next();
                            self.iter1 = self.iter1_clone.clone();
                        }
                    }
                }
                None => break None
            }
        }
    }
}

fn count_occupied(seats: &SeatMap) -> usize {
    seats
        .values()
        .cloned()
        .filter(|&seat| seat == Seat::Occupied)
        .count()
}

fn parse(s: &str) -> Result<(SeatMap, Pos), Box<dyn std::error::Error>> {
    let mut seats = SeatMap::new();

    s
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, ch)| {
                    seats.insert(
                        Pos { x, y },
                        Seat::try_from(ch).unwrap()
                    );
                })
        });

    let xmax = seats.keys().map(|&Pos { x, y: _ }| x).max().unwrap();
    let ymax = seats.keys().map(|&Pos { x: _, y }| y).max().unwrap();

    Ok((seats, Pos { x: xmax, y: ymax }))
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl From<Seat> for char {
    fn from(seat: Seat) -> char {
        use Seat::*;
        match seat {
            Floor => '.',
            Empty => 'L',
            Occupied => '#',
        }
    }
}

impl TryFrom<char> for Seat {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::Empty),
            '#' => Ok(Self::Occupied),
            _ => Err(())
        }
    }
}

#[test]
fn test_part1() {
    let eg = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL\
        ";

    let (seats, max) = parse(&eg).unwrap();
    assert_eq!(part1(&seats, &max), 37);
}

fn part2(seats: &SeatMap, max: &Pos) -> usize {
    let rest_state = game_of_life(
        seats,
        max,
        part2_iteration);

    count_occupied(&rest_state)
}

fn part2_iteration(seat: Seat, pos: &Pos, seats: &SeatMap) -> Option<Seat> {
    match seat {
        Seat::Empty if count_line_of_sight(seats, &pos) == 0 => {
            Some(Seat::Occupied)
        }

        Seat::Occupied if count_line_of_sight(seats, &pos) >= 5 => {
            Some(Seat::Empty)
        }

        _ => None
    }
}

fn count_line_of_sight(seats: &SeatMap, &Pos { x, y }: &Pos) -> usize {
    let x = x as isize;
    let y = y as isize;

    [
        (-1, -1),
        ( 0, -1),
        ( 1, -1),
        (-1,  0),
        //(0, 0),
        ( 1,  0),
        (-1,  1),
        ( 0,  1),
        ( 1,  1),
    ]
        .iter()
        .cloned()
        .filter_map(|dir| {
            let mut pos = Pos {
                x: (x + dir.0) as usize,
                y: (y + dir.1) as usize,
            };

            loop {
                match seats.get(&pos) {
                    Some(Seat::Occupied) => {
                        break Some(());
                    }
                    Some(Seat::Floor) => {
                        pos.x = (pos.x as isize + dir.0) as usize;
                        pos.y = (pos.y as isize + dir.1) as usize;
                    },
                    Some(Seat::Empty) | None => {
                        break None;
                    }
                }
            }
        })
        .count()
}

#[test]
fn test_part2() {
    let eg = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL\
        ";

    let (seats, max) = parse(&eg).unwrap();
    assert_eq!(part2(&seats, &max), 26);
}
