fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./input")?;
    let bytes = s
        .trim_end()
        .split(",")
        .map(str::parse)
        .collect::<Result<Vec<Word>, _>>()?;

    interpret(&mut bytes, &mut Default::default());
    show_bytes(&bytes);

    Ok(())
}
