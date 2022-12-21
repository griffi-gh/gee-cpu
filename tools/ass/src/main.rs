use anyhow::{Result, bail};
use std::{env, fs};

mod arch;
mod token;
use token::Tokenizer;

fn main() -> Result<()> {
  let path = match env::args().nth(1) {
    Some(x) => x,
    None => bail!("No path provided"),
  };
  let data = fs::read_to_string(path)?;
  let tokens = Tokenizer::tokenize(&data)?;
  Ok(())
}
