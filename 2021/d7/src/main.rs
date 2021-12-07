type N = i32;
struct Positions(Vec<N>);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let positions = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&positions));
    println!("Part 2: {}", part2(&positions));

    Ok(())
}

fn part1(positions: &Positions) -> N {
    let (min, max) = positions.min_max();

    let best_alignment = (min..=max)
        .min_by_key(|&pos| positions.cost_to_align(pos))
        .unwrap();

    positions.cost_to_align(best_alignment)
}

fn part2(positions: &Positions) -> N {
    let (min, max) = positions.min_max();

    let best_alignment = (min..=max)
        .min_by_key(|&pos| positions.cost_to_align2(pos))
        .unwrap();

    positions.cost_to_align2(best_alignment)
}

impl Positions {
    fn cost_to_align(&self, pos: N) -> N {
        self.0
            .iter()
            .map(|&x| (x - pos).abs())
            .sum()
    }

    fn cost_to_align2(&self, pos: N) -> N {
        self.0
            .iter()
            .map(|&x| (x - pos).abs())
            .map(|cost| (1..=cost).sum::<N>())
            .sum()
    }

    fn min_max(&self) -> (N, N) {
        (*self.0.iter().min().unwrap(), *self.0.iter().max().unwrap())
    }
}

impl std::str::FromStr for Positions {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "couldn't parse int")?,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EG: &'static str = "\
        16,1,2,0,4,2,7,1,2,14\
    ";

    #[test]
    fn test_part1() {
        let positions = EG.parse().unwrap();
        assert_eq!(part1(&positions), 37);
    }

    #[test]
    fn test_part2() {
        let positions = EG.parse().unwrap();
        assert_eq!(part2(&positions), 168);
    }
}
