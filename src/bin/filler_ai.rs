use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    // Just output a move immediately
    println!("0 0");
    io::stdout().flush().ok();
    Ok(())
}
