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


    let mut total_questions = 0;
    for group in groups {
        let mut count = HashMap::new();
        for person in group.iter() {
            for answer in person.chars() {
                let entry = count.entry(answer).or_insert(0);
                *entry += 1;
            }
        }

        total_questions += count.len();
    }

    println!("{}", total_questions);

    Ok(())
}
