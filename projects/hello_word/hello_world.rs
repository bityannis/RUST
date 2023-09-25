//Single rust file. Compile with rustc hello_world.rs

use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Hello World");
    writeln!(io::stdout(), "Hello World!")?;
    Ok(())
}
