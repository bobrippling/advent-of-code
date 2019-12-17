pub fn with_tty_io<F, R>(f: F) -> R
    where F: FnOnce() -> R
{
    let mut stty = Command::new("stty");
    stty.arg("-echo").arg("-icanon");
    stty.status().expect("stty failed");

    let r = f();

    let mut stty = Command::new("stty");
    stty.arg("echo").arg("icanon");
    stty.status().expect("stty failed");

    r
}

pub fn flush_out_read_ch() -> char {
    let stdout = io::stdout();
    stdout.lock().flush().unwrap();

    let mut reader = io::stdin();
    let mut buffer = [0; 1]; // read exactly one byte

    reader.read_exact(&mut buffer).unwrap();

    buffer[0] as char
}
