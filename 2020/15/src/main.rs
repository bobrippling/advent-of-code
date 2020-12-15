use std::fs;
use std::collections::HashMap;

type N = i32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;

    println!("{}", part1(&s)?);
    println!("{}", part2(&s)?);

    Ok(())
}

#[derive(Copy, Clone)]
enum Ent {
    SeenOnce { turn: N },
    SeenTwice { turn_old: N, turn_new: N },
}

fn part1(input: &str) -> Result<N, Box<dyn std::error::Error>> {
    find_nth(input, 2020)
}

fn find_nth(input: &str, target_turn: N) -> Result<N, Box<dyn std::error::Error>> {
    let nums = input.split(',').map(str::trim).map(str::parse).collect::<Result<Vec<_>, _>>()?;

    let mut history = HashMap::new();
    let mut turn = 1;
    let mut last = -1;

    for num in nums.iter().cloned() {
        history.insert(num, Ent::SeenOnce { turn });

        turn += 1;
        last = num;
    }

    loop {
        let new = match history.get(&last).map(|&t| t) {
            Some(Ent::SeenOnce { .. }) => {
                let r = 0;
                history.update(r, turn, &|n| Ent::SeenTwice { turn_old: n, turn_new: turn });
                r
            }
            Some(Ent::SeenTwice { turn_old, turn_new }) => {
                let r = turn_new - turn_old;
                history.update(r, turn, &|n| Ent::SeenTwice { turn_old: n, turn_new: turn });
                r
            }
            None => {
                history.insert(0, Ent::SeenOnce { turn });
                0
            }
        };

        if turn == target_turn {
            break Ok(new);
        }

        turn += 1;
        last = new;
    }
}

trait Updatable {
    fn update(&mut self, n: N, turn: N, when_seen: &dyn Fn(N) -> Ent);
}

impl Updatable for HashMap<N, Ent> {
    fn update(&mut self, n: N, turn: N, when_seen: &dyn Fn(N) -> Ent) {
        match self.get(&n).map(|&t| t) {
            Some(Ent::SeenOnce { turn }) | Some(Ent::SeenTwice { turn_new: turn, .. }) => {
                self.insert(n, when_seen(turn));
            }
            None => {
                self.insert(n, Ent::SeenOnce { turn });
            }
        };
    }
}

#[test]
fn test_part1() {
    let input = "0,3,6";

    assert_eq!(
        part1(&input).unwrap(),
        436);
}

fn part2(input: &str) -> Result<N, Box<dyn std::error::Error>> {
    find_nth(input, 30000000)
}

#[test]
fn test_part1_2() {
    assert_eq!(part1(&"1,3,2").unwrap(), 1);
}
#[test]
fn test_part1_3() {
    assert_eq!(part1(&"2,1,3").unwrap(), 10);
}
#[test]
fn test_part1_4() {
    assert_eq!(part1(&"1,2,3").unwrap(), 27);
}
#[test]
fn test_part1_5() {
    assert_eq!(part1(&"2,3,1").unwrap(), 78);
}
#[test]
fn test_part1_6() {
    assert_eq!(part1(&"3,2,1").unwrap(), 438);
}
#[test]
fn test_part1_7() {
    assert_eq!(part1(&"3,1,2").unwrap(), 1836);
}


#[test]
fn test_part2() {
    assert_eq!(part2(&"0,3,6").unwrap(), 175594);
    assert_eq!(part2(&"1,3,2").unwrap(), 2578);
    assert_eq!(part2(&"2,1,3").unwrap(), 3544142);
    assert_eq!(part2(&"1,2,3").unwrap(), 261214);
    assert_eq!(part2(&"2,3,1").unwrap(), 6895259);
    assert_eq!(part2(&"3,2,1").unwrap(), 18);
    assert_eq!(part2(&"3,1,2").unwrap(), 362);
}
