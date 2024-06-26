use std::{
    fs::File,
    io::{self, Read},
};

use yet_another_compiler::compiler::compiler;

fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").expect("Cannot read file");
    let mut input = String::new();

    f.read_to_string(&mut input)?;
    let _ = compiler(input);

    Ok(())
}
