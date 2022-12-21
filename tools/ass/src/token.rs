use anyhow::{Result};

pub struct CodePosition {
  pub row: usize,
  pub col: usize,
}
impl CodePosition {
  pub fn new(row: usize, col: usize) -> Self {
    Self { row, col }
  }
}
impl Default for CodePosition {
  fn default() -> Self {
    Self::new(0, 0)
  }
}
impl From<(usize, usize)> for CodePosition {
  fn from(value: (usize, usize)) -> Self {
    Self::new(value.0, value.1)
  }
}
impl From<CodePosition> for (usize, usize) {
  fn from(value: CodePosition) -> Self {
    (value.row, value.col)
  }
}

pub enum TokenType<'a> {
  Instruction,
  StringLiteral(&'a str),
  IntegerLiteral(isize),
  Marker(&'a str, ),

}

pub struct Token<'a> {
  pub token: TokenType<'a>
}

pub struct Tokenizer<'a> {
  code: &'a str,
  tokens: Vec<Token<'a>>,
  position: CodePosition
}
impl<'a> Tokenizer<'a> {
  pub fn new(code: &'a str) -> Self {
    Self {
      code,
      tokens: Vec::new(),
      position: CodePosition::default()
    }
  }
  pub fn step(&mut self) -> Result<()> {
    Ok(())
  }
  pub fn run(&mut self) -> Result<()> {
    loop {
      self.step()?;
    }
    Ok(())
  }
  pub fn finish(self) -> Vec<Token<'a>> {
    self.tokens
  }
  // pub fn tokenize(data: &str) -> Result<Vec<Token>> {
  //   let mut tokens = Vec::new();
  //   let mut position 
  //   loop {
  //     data.
  //   }
  //   Ok(tokens)
  // }
}
