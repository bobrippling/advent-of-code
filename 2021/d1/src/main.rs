fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Vec<u32>) -> u32 {
    input
        .iter()
        .copied()
        .fold((0, None), |(incs, last), depth| match last {
            None => (0, Some(depth)),
            Some(last) => {
                if depth > last {
                    (incs + 1, Some(depth))
                } else {
                    (incs, Some(depth))
                }
            }
        })
        .0
}

fn part2(input: &Vec<u32>) -> u32 {
    input
        .windows(3)
        .fold((0, None), |(incs, last): (u32, Option<u32>), depths| {
            let depth = depths.iter().sum();

            match last {
                None => (0, Some(depth)),
                Some(last) => {
                    if depth > last {
                        (incs + 1, Some(depth))
                    } else {
                        (incs, Some(depth))
                    }
                }
            }
        })
        .0
}
