use anyhow::{Result};
use crate::arch::Register;

/// Represents position of a character in code
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodePosition {
  pub char: usize,
  pub row: usize,
  pub col: usize,
}
impl CodePosition {
  /// Create a new [`CodePosition`]
  pub fn new(char: usize, row: usize, col: usize) -> Self {
    Self { char, row, col }
  }
  /// Creates a new [`CodePosition`] pointing to the next character
  pub fn next(&self) -> Self {
    Self {
      char: self.char + 1,
      col: self.col + 1,
      ..*self
    }
  }
  /// Creates a new [`CodePosition`] pointing to the start of the next row
  pub fn next_row(&self) -> Self {
    Self {
      char: self.char + 1,
      col: 0,
      row: self.row + 1,
      ..*self
    }
  }
  /// Calls [`CodePosition::next_row`] if the character is newline, otherwise [`CodePosition::next`]
  pub fn next_auto(&self, chr: char) -> Self {
    match chr {
      '\n' => self.next_row(),
      _ => self.next()
    }
  }
}
impl Default for CodePosition {
  fn default() -> Self {
    Self::new(0, 0, 0)
  }
}

#[derive(Clone, Copy, Debug)]
pub enum TokenType<'a> {
  Instruction(&'a str),
  StringLiteral(&'a str),
  IntegerLiteral(isize),
  SymbolLiteral(&'a str),
  Symbol(&'a str),
  RegisterPointer(Register),
  Whitespace,
  Eof,
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
  pub token: TokenType<'a>
}

#[derive(Clone)]
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
  fn peek(&self, offset: isize) -> Option<char> {
    self.code.chars().nth(self.position.char)
  }
  fn eat(&mut self) -> Option<char> {
    let chr = self.peek(0)?;
    self.position = self.position.next_auto(chr);
    Some(chr)
  }
  pub fn step(&mut self) -> Result<()> {
    Ok(())
  }
  /// Run tokenizer until the end of file (EOF)
  pub fn run(&mut self) -> Result<()> {
    loop {
      self.step()?;
    }
    Ok(())
  }
  pub fn finish(self) -> Vec<Token<'a>> {
    self.tokens
  }
}
