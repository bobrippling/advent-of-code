mod parse;

fn input() -> Word {
    use std::io;
    eprintln!("input");
    let mut line = String::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("error reading stdin: {}", e);
                std::process::exit(1);
            }
        };

        let line = line.trim_end();

        match line.parse::<Word>() {
            Ok(i) => return i,
            Err(_) => {
                eprintln!("couldn't parse {}, try again", line);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = parse::bytes("./input")?;

    interpret(&mut bytes, &mut Default::default());
    show_bytes(&bytes);

    Ok(())
}
