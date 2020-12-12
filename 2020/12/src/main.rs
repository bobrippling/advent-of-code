use std::fs;

type Step = i32;
type Degrees = i16;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;

    let cmds = parse(&s)?;
    println!("Part 1: {}", part1(&cmds));
    println!("Part 2: {}", part2(&cmds));

    Ok(())
}

struct Ship {
    facing: Degrees,
    pos: Pos,
}

impl Ship {
    fn new() -> Self {
        Self::new_at(Pos::origin())
    }

    fn new_at(pos: Pos) -> Self {
        Self {
            facing: 90,
            pos,
        }
    }

    fn manhattan_from_origin(&self) -> Step {
        self.pos.x.abs() + self.pos.y.abs()
    }

    fn rotate(&mut self, deg: Degrees) {
        self.facing += deg;
        while self.facing < 0 {
            self.facing += 360
        }
        while self.facing >= 360 {
            self.facing -= 360
        }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

enum Command {
    DirMove(Direction, Step),
    TurnLeft(Degrees),
    TurnRight(Degrees),
    Forward(Step),
}

#[derive(Copy, Clone)]
struct Pos {
    x: Step,
    y: Step,
}

impl Pos {
    fn origin() -> Self {
        Pos { x: 0, y: 0 }
    }
}

fn parse(s: &str) -> Result<Vec<Command>, Box<dyn std::error::Error>> {
    let q = s.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| -> Result<_, Box<dyn std::error::Error>> {
            let (ch, n) = l.split_at(1);
            let n = n.parse()?;

            let cmd = match ch {
                "N" => Command::DirMove(Direction::North, n),
                "S" => Command::DirMove(Direction::South, n),
                "E" => Command::DirMove(Direction::East, n),
                "W" => Command::DirMove(Direction::West, n),

                "L" => Command::TurnLeft(n as _),
                "R" => Command::TurnRight(n as _),
                "F" => Command::Forward(n as _),
                _ => {
                    return Err("invalid command".into());
                }
            };
            Ok(cmd)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(q)
}

fn part1(cmds: &Vec<Command>) -> Step {
    let mut ship = Ship::new();

    for cmd in cmds {
        match cmd {
            Command::DirMove(dir, n) => {
                match dir {
                    Direction::North => ship.pos.y -= n,
                    Direction::South => ship.pos.y += n,
                    Direction::East => ship.pos.x += n,
                    Direction::West => ship.pos.x -= n,
                }
            }
            Command::TurnLeft(deg) => ship.rotate(-deg),
            Command::TurnRight(deg) => ship.rotate(*deg),
            Command::Forward(n) => {
                let facing = ship.facing;
                if facing == 0 {
                    ship.pos.y -= n;
                } else if facing == 90 {
                    ship.pos.x += n;
                } else if facing == 180 {
                    ship.pos.y += n;
                } else if facing == 270 {
                    ship.pos.x -= n;
                } else {
                    panic_not_90_degrees();
                }
            }
        }
    }

    ship.manhattan_from_origin()
}

fn panic_not_90_degrees() -> ! {
    panic!("not a 90 degree angle!");
}

#[test]
fn test_part1() {
    let s = "\
F10
N3
F7
R90
F11";
    let cmds = parse(&s).unwrap();
    assert_eq!(part1(&cmds), 25);
}

fn part2(cmds: &Vec<Command>) -> Step {
    let mut ship = Ship::new();
    let mut waypoint = Ship::new_at(Pos { x: 10, y: -1 });

    for cmd in cmds {
        match cmd {
            Command::DirMove(dir, n) => {
                match dir {
                    Direction::North => waypoint.pos.y -= n,
                    Direction::South => waypoint.pos.y += n,
                    Direction::East => waypoint.pos.x += n,
                    Direction::West => waypoint.pos.x -= n,
                }
            }
            Command::TurnLeft(deg) => {
                let deg = *deg;
                if deg == 90 {
                    let x = waypoint.pos.y;
                    let y = -waypoint.pos.x;

                    waypoint.pos.x = x;
                    waypoint.pos.y = y;
                } else if deg == 180 {
                    waypoint.pos.x *= -1;
                    waypoint.pos.y *= -1;
                } else if deg == 270 {
                    let x = -waypoint.pos.y;
                    let y = waypoint.pos.x;

                    waypoint.pos.x = x;
                    waypoint.pos.y = y;
                } else {
                    panic_not_90_degrees();
                }
            },
            Command::TurnRight(deg) => {
                let deg = *deg;

                if deg == 90 {
                    let x = -waypoint.pos.y;
                    let y = waypoint.pos.x;

                    waypoint.pos.x = x;
                    waypoint.pos.y = y;
                } else if deg == 180 {
                    waypoint.pos.x *= -1;
                    waypoint.pos.y *= -1;
                } else if deg == 270 {
                    let x = waypoint.pos.y;
                    let y = -waypoint.pos.x;

                    waypoint.pos.x = x;
                    waypoint.pos.y = y;
                } else {
                    panic_not_90_degrees();
                }
            },
            Command::Forward(n) => {
                let step = Pos {
                    x: waypoint.pos.x * n,
                    y: waypoint.pos.y * n,
                };

                ship.pos.x += step.x;
                ship.pos.y += step.y;
            }
        }
    }

    ship.manhattan_from_origin()
}

#[test]
fn test_part2() {
    let s = "\
F10
N3
F7
R90
F11";
    let cmds = parse(&s).unwrap();
    assert_eq!(part2(&cmds), 286);
}
