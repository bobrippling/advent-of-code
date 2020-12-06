use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input.txt")?;
    let mut groups = vec![];
    let mut group = vec![];

    s
        .split("\n")
        .for_each(|line| {
            if line.is_empty() {
                groups.push(group.clone());
                group.clear();
            } else {
                group.push(line);
            }
        });


    let mut part1_total_questions = 0;
    let mut part2_total_questions = 0;

    for group in groups {
        let mut count = HashMap::new();
        for person in group.iter() {
            for answer in person.chars() {
                let entry = count.entry(answer).or_insert(0);
                *entry += 1;
            }
        }

        part1_total_questions += count.len();
        for &n in count.values() {
            if n == group.len() {
                part2_total_questions += 1;
            }
        }
    }

    println!("Part 1: {}", part1_total_questions);
    println!("Part 2: {}", part2_total_questions);

    Ok(())
}
