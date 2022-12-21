use anyhow::{Result, bail};
use std::{env, fs};

mod token;
use token::Tokenizer;

fn main() -> Result<()> {
  let path = match env::args().nth(1) {
    Some(x) => x,
    None => bail!("No path provided"),
  };
  let data = fs::read_to_string(path)?;
  let tokenizer = Tokenizer::new(&data).finish();
  Ok(())
}
