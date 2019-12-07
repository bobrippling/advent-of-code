fn show_bytes(bytes: &[Word]) {
    for b in bytes {
        print!("{},", b);
    }
    println!("");
}

#[allow(dead_code)]
fn part1(bytes_slice: &[Word]) {
    let mut bytes = Vec::new();
    bytes.resize(bytes_slice.len(), 0);
    bytes.copy_from_slice(bytes_slice);

    bytes[1] = 12;
    bytes[2] = 2;

    println!("input");
    show_bytes(&bytes);

    interpret(&mut bytes, &mut Default::default());

    println!("output");
    show_bytes(&bytes);
}

#[allow(dead_code)]
fn part2(bytes_slice: &[Word]) {
    fn find(bytes_slice: &[Word]) -> Option<Word> {
        let mut bytes = Vec::new();
        bytes.resize(bytes_slice.len(), 0);

        let desired = 19690720;

        for noun in 0..=99 {
            for verb in 0..=99 {
                bytes.copy_from_slice(bytes_slice);
                bytes[1] = noun;
                bytes[2] = verb;

                interpret(&mut bytes, &mut Default::default());

                let output = bytes[0];

                //println!("{} and {} give {}", noun, verb, output);

                if output == desired {
                    return Some(100 * noun + verb);
                }
            }
        }

        None
    }

    match find(bytes_slice) {
        Some(answer) => println!("found: {}", answer),
        None => println!("Couldn't find match"),
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input-day2")?;
    let bytes = s
        .trim_end()
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<Word>, _>>()?;

    //part1(&bytes);
    //part2(&bytes);

    Ok(())
}
