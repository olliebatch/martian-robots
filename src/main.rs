use std::io::{self, Read};

fn main() -> Result<(), anyhow::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    println!("{}", buffer);

    Ok(())
}
