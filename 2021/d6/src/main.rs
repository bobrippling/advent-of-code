fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fish = std::fs::read_to_string("input.txt")?.parse()?;

    println!("Part 1: {}", part1(&fish, 80));
    println!("Part 1: {} // fast technique", part2(&fish, 80));
    println!("Part 2: {}", part2(&fish, 256));

    Ok(())
}

#[derive(Clone)]
struct Fish(Vec<usize>);

type N = u64;

#[derive(Clone, Default)]
struct FastFish([N; 9]);

fn part1(fish: &Fish, iterations: usize) -> N {
    let mut fish = fish.clone();

    for _ in 0..iterations {
        fish.run();
    }

    fish.0.len() as _
}

fn part2(fish: &Fish, iterations: usize) -> N {
    let mut fish: FastFish = fish.into();

    for _ in 0..iterations {
        fish.run();
    }

    fish.count()
}

impl Fish {
    fn run(&mut self) {
        let mut new = Vec::new();

        for fish in &mut self.0 {
            if *fish == 0 {
                new.push(8);
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }

        self.0.append(&mut new);
    }
}

impl FastFish {
    fn run(&mut self) {
        let mut new = Self::default();

        for age in 0..9 {
            let n = *self.fish_aged(age);
            if age == 0 {
                *new.fish_aged(6) += n;
                *new.fish_aged(8) += n;
            } else {
                *new.fish_aged(age - 1) += n;
            }
        }

        *self = new;
    }

    fn fish_aged(&mut self, age: usize) -> &mut N {
        &mut self.0[age]
    }

    fn count(&self) -> N {
        self.0.iter().copied().sum()
    }
}

impl std::fmt::Debug for FastFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (age, &count) in self.0.iter().enumerate() {
            for _ in 0..count {
                write!(f, "{},", age)?;
            }
        }
        Ok(())
    }
}

impl From<&Fish> for FastFish {
    fn from(f: &Fish) -> Self {
        let mut fastfish = Self::default();

        for &age in &f.0 {
            *fastfish.fish_aged(age) += 1;
        }

        fastfish
    }
}

impl std::str::FromStr for Fish {
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

    static EG: &'static str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let fish = EG.parse().unwrap();

        assert_eq!(part1(&fish, 18), 26);
        assert_eq!(part1(&fish, 80), 5934);

        assert_eq!(part2(&fish, 18), 26);
        assert_eq!(part2(&fish, 80), 5934);
    }
}
