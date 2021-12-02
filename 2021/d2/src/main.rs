fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commands = std::fs::read_to_string("input.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands));

    Ok(())
}

enum Move {
    Up(u32),
    Down(u32),
    Forward(u32),
}

impl std::str::FromStr for Move {
    type Err = &'static str;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = l.split(' ').collect();
        match &parts[..] {
            [dir, amt] => {
                let amt = amt.parse().or(Err("not a number"))?;

                match dir {
                    &"up" => Ok(Self::Up(amt)),
                    &"down" => Ok(Self::Down(amt)),
                    &"forward" => Ok(Self::Forward(amt)),
                    _ => Err("invalid field"),
                }
            },
            _ => Err("expected two fields"),
        }
    }
}

#[derive(Default)]
struct Pos {
    depth: u32,
    hpos: u32,
}

fn part1(commands: &Vec<Move>) -> u32 {
    let pos = commands.iter().fold(Pos::default(), |pos, cmd| {
        use Move::*;
        match cmd {
            Up(u) => Pos {
                depth: pos.depth - u,
                ..pos
            },
            Down(d) => Pos {
                depth: pos.depth + d,
                ..pos
            },
            Forward(f) => Pos {
                hpos: pos.hpos + f,
                ..pos
            },
        }
    });

    pos.hpos * pos.depth
}

#[derive(Default)]
struct Pos2 {
    depth: u32,
    hpos: u32,
    aim: u32,
}

fn part2(commands: &Vec<Move>) -> u32 {
    let pos = commands.iter().fold(Pos2::default(), |pos, cmd| {
        use Move::*;
        match cmd {
            Up(u) => Pos2 {
                aim: pos.aim - u,
                ..pos
            },
            Down(d) => Pos2 {
                aim: pos.aim + d,
                ..pos
            },
            Forward(x) => Pos2 {
                hpos: pos.hpos + x,
                depth: pos.depth + x * pos.aim,
                ..pos
            },
        }
    });

    pos.hpos * pos.depth
}
